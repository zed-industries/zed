// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Different optimizations for pest's ASTs.

use crate::ast::*;
use std::collections::HashMap;

#[cfg(test)]
macro_rules! box_tree {
    ( $node:ident( $(                      $child:ident( $($args:tt)* )     ),+ ) ) => (
      $node      ( $( Box::new( box_tree!( $child      ( $($args   )* ) ) ) ),+ )
    );
    ($expr:expr) => ($expr);
}

mod concatenator;
mod factorizer;
mod lister;
mod restorer;
mod rotater;
mod skipper;
mod unroller;

/// Takes pest's ASTs and optimizes them
pub fn optimize(rules: Vec<Rule>) -> Vec<OptimizedRule> {
    let map = to_hash_map(&rules);
    let optimized: Vec<OptimizedRule> = rules
        .into_iter()
        .map(rotater::rotate)
        .map(|rule| skipper::skip(rule, &map))
        .map(unroller::unroll)
        .map(concatenator::concatenate)
        .map(factorizer::factor)
        .map(lister::list)
        .map(rule_to_optimized_rule)
        .collect();

    let optimized_map = to_optimized_hash_map(&optimized);
    optimized
        .into_iter()
        .map(|rule| restorer::restore_on_err(rule, &optimized_map))
        .collect()
}

fn rule_to_optimized_rule(rule: Rule) -> OptimizedRule {
    fn to_optimized(expr: Expr) -> OptimizedExpr {
        match expr {
            Expr::Str(string) => OptimizedExpr::Str(string),
            Expr::Insens(string) => OptimizedExpr::Insens(string),
            Expr::Range(start, end) => OptimizedExpr::Range(start, end),
            Expr::Ident(ident) => OptimizedExpr::Ident(ident),
            Expr::PeekSlice(start, end) => OptimizedExpr::PeekSlice(start, end),
            Expr::PosPred(expr) => OptimizedExpr::PosPred(Box::new(to_optimized(*expr))),
            Expr::NegPred(expr) => OptimizedExpr::NegPred(Box::new(to_optimized(*expr))),
            Expr::Seq(lhs, rhs) => {
                OptimizedExpr::Seq(Box::new(to_optimized(*lhs)), Box::new(to_optimized(*rhs)))
            }
            Expr::Choice(lhs, rhs) => {
                OptimizedExpr::Choice(Box::new(to_optimized(*lhs)), Box::new(to_optimized(*rhs)))
            }
            Expr::Opt(expr) => OptimizedExpr::Opt(Box::new(to_optimized(*expr))),
            Expr::Rep(expr) => OptimizedExpr::Rep(Box::new(to_optimized(*expr))),
            Expr::Skip(strings) => OptimizedExpr::Skip(strings),
            Expr::Push(expr) => OptimizedExpr::Push(Box::new(to_optimized(*expr))),
            #[cfg(feature = "grammar-extras")]
            Expr::PushLiteral(string) => OptimizedExpr::PushLiteral(string),
            #[cfg(feature = "grammar-extras")]
            Expr::NodeTag(expr, tag) => OptimizedExpr::NodeTag(Box::new(to_optimized(*expr)), tag),
            #[cfg(feature = "grammar-extras")]
            Expr::RepOnce(expr) => OptimizedExpr::RepOnce(Box::new(to_optimized(*expr))),
            #[cfg(not(feature = "grammar-extras"))]
            Expr::RepOnce(_) => unreachable!("No valid transformation to OptimizedRule"),
            Expr::RepExact(..) | Expr::RepMin(..) | Expr::RepMax(..) | Expr::RepMinMax(..) => {
                unreachable!("No valid transformation to OptimizedRule")
            }
        }
    }

    OptimizedRule {
        name: rule.name,
        ty: rule.ty,
        expr: to_optimized(rule.expr),
    }
}

macro_rules! to_hash_map {
    ($func_name:ident, $rule:ty, $expr:ty) => {
        fn $func_name(rules: &[$rule]) -> HashMap<String, $expr> {
            rules
                .iter()
                .map(|r| (r.name.clone(), r.expr.clone()))
                .collect()
        }
    };
}
to_hash_map!(to_hash_map, Rule, Expr);
to_hash_map!(to_optimized_hash_map, OptimizedRule, OptimizedExpr);

/// The optimized version of the pest AST's `Rule`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptimizedRule {
    /// The name of the rule.
    pub name: String,
    /// The type of the rule.
    pub ty: RuleType,
    /// The optimized expression of the rule.
    pub expr: OptimizedExpr,
}

/// The optimized version of the pest AST's `Expr`.
///
/// # Warning: Semantic Versioning
/// There may be non-breaking changes to the meta-grammar
/// between minor versions. Those non-breaking changes, however,
/// may translate into semver-breaking changes due to the additional variants
/// propaged from the `Rule` enum. This is a known issue and will be fixed in the
/// future (e.g. by increasing MSRV and non_exhaustive annotations).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptimizedExpr {
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
    PosPred(Box<OptimizedExpr>),
    /// Negative lookahead; matches if expression doesn't match, without making progress, e.g. `!e`
    NegPred(Box<OptimizedExpr>),
    /// Matches a sequence of two expressions, e.g. `e1 ~ e2`
    Seq(Box<OptimizedExpr>, Box<OptimizedExpr>),
    /// Matches either of two expressions, e.g. `e1 | e2`
    Choice(Box<OptimizedExpr>, Box<OptimizedExpr>),
    /// Optionally matches an expression, e.g. `e?`
    Opt(Box<OptimizedExpr>),
    /// Matches an expression zero or more times, e.g. `e*`
    Rep(Box<OptimizedExpr>),
    /// Matches an expression one or more times, e.g. `e+`
    #[cfg(feature = "grammar-extras")]
    RepOnce(Box<OptimizedExpr>),
    /// Continues to match expressions until one of the strings in the `Vec` is found
    Skip(Vec<String>),
    /// Matches an expression and pushes it to the stack, e.g. `push(e)`
    Push(Box<OptimizedExpr>),
    /// Pushes a literal string to the stack, e.g. `push_literal("a")`
    #[cfg(feature = "grammar-extras")]
    PushLiteral(String),
    /// Matches an expression and assigns a label to it, e.g. #label = exp
    #[cfg(feature = "grammar-extras")]
    NodeTag(Box<OptimizedExpr>, String),
    /// Restores an expression's checkpoint
    RestoreOnErr(Box<OptimizedExpr>),
}

impl OptimizedExpr {
    /// Returns a top-down iterator over the `OptimizedExpr`.
    pub fn iter_top_down(&self) -> OptimizedExprTopDownIterator {
        OptimizedExprTopDownIterator::new(self)
    }

    /// Applies `f` to the `OptimizedExpr` top-down.
    pub fn map_top_down<F>(self, mut f: F) -> OptimizedExpr
    where
        F: FnMut(OptimizedExpr) -> OptimizedExpr,
    {
        fn map_internal<F>(expr: OptimizedExpr, f: &mut F) -> OptimizedExpr
        where
            F: FnMut(OptimizedExpr) -> OptimizedExpr,
        {
            let expr = f(expr);

            match expr {
                OptimizedExpr::PosPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::PosPred(mapped)
                }
                OptimizedExpr::NegPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::NegPred(mapped)
                }
                OptimizedExpr::Seq(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    OptimizedExpr::Seq(mapped_lhs, mapped_rhs)
                }
                OptimizedExpr::Choice(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    OptimizedExpr::Choice(mapped_lhs, mapped_rhs)
                }
                OptimizedExpr::Rep(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Rep(mapped)
                }
                OptimizedExpr::Opt(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Opt(mapped)
                }
                OptimizedExpr::Push(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Push(mapped)
                }
                expr => expr,
            }
        }

        map_internal(self, &mut f)
    }

    /// Applies `f` to the `OptimizedExpr` bottom-up.
    pub fn map_bottom_up<F>(self, mut f: F) -> OptimizedExpr
    where
        F: FnMut(OptimizedExpr) -> OptimizedExpr,
    {
        fn map_internal<F>(expr: OptimizedExpr, f: &mut F) -> OptimizedExpr
        where
            F: FnMut(OptimizedExpr) -> OptimizedExpr,
        {
            let mapped = match expr {
                OptimizedExpr::PosPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::PosPred(mapped)
                }
                OptimizedExpr::NegPred(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::NegPred(mapped)
                }
                OptimizedExpr::Seq(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    OptimizedExpr::Seq(mapped_lhs, mapped_rhs)
                }
                OptimizedExpr::Choice(lhs, rhs) => {
                    let mapped_lhs = Box::new(map_internal(*lhs, f));
                    let mapped_rhs = Box::new(map_internal(*rhs, f));
                    OptimizedExpr::Choice(mapped_lhs, mapped_rhs)
                }
                OptimizedExpr::Rep(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Rep(mapped)
                }
                OptimizedExpr::Opt(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Opt(mapped)
                }
                OptimizedExpr::Push(expr) => {
                    let mapped = Box::new(map_internal(*expr, f));
                    OptimizedExpr::Push(mapped)
                }
                expr => expr,
            };

            f(mapped)
        }

        map_internal(self, &mut f)
    }
}

impl core::fmt::Display for OptimizedExpr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OptimizedExpr::Str(s) => write!(f, "{:?}", s),
            OptimizedExpr::Insens(s) => write!(f, "^{:?}", s),
            OptimizedExpr::Range(start, end) => {
                let start = start.chars().next().expect("Empty range start.");
                let end = end.chars().next().expect("Empty range end.");
                write!(f, "({:?}..{:?})", start, end)
            }
            OptimizedExpr::Ident(id) => write!(f, "{}", id),
            OptimizedExpr::PeekSlice(start, end) => match end {
                Some(end) => write!(f, "PEEK[{}..{}]", start, end),
                None => write!(f, "PEEK[{}..]", start),
            },
            OptimizedExpr::PosPred(expr) => write!(f, "&{}", expr.as_ref()),
            OptimizedExpr::NegPred(expr) => write!(f, "!{}", expr.as_ref()),
            OptimizedExpr::Seq(lhs, rhs) => {
                let mut nodes = Vec::new();
                nodes.push(lhs);
                let mut current = rhs;
                while let OptimizedExpr::Seq(lhs, rhs) = current.as_ref() {
                    nodes.push(lhs);
                    current = rhs;
                }
                nodes.push(current);
                let sequence = nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<_>>()
                    .join(" ~ ");
                write!(f, "({})", sequence)
            }
            OptimizedExpr::Choice(lhs, rhs) => {
                let mut nodes = Vec::new();
                nodes.push(lhs);
                let mut current = rhs;
                while let OptimizedExpr::Choice(lhs, rhs) = current.as_ref() {
                    nodes.push(lhs);
                    current = rhs;
                }
                nodes.push(current);
                let sequence = nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<_>>()
                    .join(" | ");
                write!(f, "({})", sequence)
            }
            OptimizedExpr::Opt(expr) => write!(f, "{}?", expr),
            OptimizedExpr::Rep(expr) => write!(f, "{}*", expr),
            #[cfg(feature = "grammar-extras")]
            OptimizedExpr::RepOnce(expr) => write!(f, "{}+", expr),
            OptimizedExpr::Skip(strings) => {
                let strings = strings
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<_>>()
                    .join(" | ");
                write!(f, "(!({}) ~ ANY)*", strings)
            }
            OptimizedExpr::Push(expr) => write!(f, "PUSH({})", expr),
            #[cfg(feature = "grammar-extras")]
            OptimizedExpr::PushLiteral(s) => write!(f, "PUSH_LITERAL({:?})", s),
            #[cfg(feature = "grammar-extras")]
            OptimizedExpr::NodeTag(expr, tag) => {
                write!(f, "(#{} = {})", tag, expr)
            }
            OptimizedExpr::RestoreOnErr(expr) => core::fmt::Display::fmt(expr.as_ref(), f),
        }
    }
}

/// A top-down iterator over an `OptimizedExpr`.
pub struct OptimizedExprTopDownIterator {
    current: Option<OptimizedExpr>,
    next: Option<OptimizedExpr>,
    right_branches: Vec<OptimizedExpr>,
}

impl OptimizedExprTopDownIterator {
    /// Creates a new top down iterator from an `OptimizedExpr`.
    pub fn new(expr: &OptimizedExpr) -> Self {
        let mut iter = OptimizedExprTopDownIterator {
            current: None,
            next: None,
            right_branches: vec![],
        };
        iter.iterate_expr(expr.clone());
        iter
    }

    fn iterate_expr(&mut self, expr: OptimizedExpr) {
        self.current = Some(expr.clone());
        match expr {
            OptimizedExpr::Seq(lhs, rhs) => {
                self.right_branches.push(*rhs);
                self.next = Some(*lhs);
            }
            OptimizedExpr::Choice(lhs, rhs) => {
                self.right_branches.push(*rhs);
                self.next = Some(*lhs);
            }
            OptimizedExpr::PosPred(expr)
            | OptimizedExpr::NegPred(expr)
            | OptimizedExpr::Rep(expr)
            | OptimizedExpr::Opt(expr)
            | OptimizedExpr::Push(expr) => {
                self.next = Some(*expr);
            }
            _ => {
                self.next = None;
            }
        }
    }
}

impl Iterator for OptimizedExprTopDownIterator {
    type Item = OptimizedExpr;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current.take();

        if let Some(expr) = self.next.take() {
            self.iterate_expr(expr);
        } else if let Some(expr) = self.right_branches.pop() {
            self.iterate_expr(expr);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Normal,
                expr: box_tree!(Choice(
                    Choice(
                        Choice(Str(String::from("a")), Str(String::from("b"))),
                        Str(String::from("c"))
                    ),
                    Str(String::from("d"))
                )),
            }]
        };
        let rotated = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Normal,
                expr: box_tree!(Choice(
                    Str(String::from("a")),
                    Choice(
                        Str(String::from("b")),
                        Choice(Str(String::from("c")), Str(String::from("d")))
                    )
                )),
            }]
        };

        assert_eq!(optimize(rules), rotated);
    }

    #[test]
    fn skip() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Rep(Seq(
                    NegPred(Choice(Str(String::from("a")), Str(String::from("b")))),
                    Ident(String::from("ANY"))
                ))),
            }]
        };
        let skipped = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: OptimizedExpr::Skip(vec![String::from("a"), String::from("b")]),
        }];

        assert_eq!(optimize(rules), skipped);
    }

    #[test]
    fn concat_strings() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(
                    Seq(Str(String::from("a")), Str(String::from("b"))),
                    Seq(Str(String::from("c")), Str(String::from("d")))
                )),
            }]
        };
        let concatenated = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: OptimizedExpr::Str(String::from("abcd")),
        }];

        assert_eq!(optimize(rules), concatenated);
    }

    #[test]
    fn unroll_loop_exact() {
        let rules = vec![Rule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: Expr::RepExact(Box::new(Expr::Ident(String::from("a"))), 3),
        }];
        let unrolled = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(
                    Ident(String::from("a")),
                    Seq(Ident(String::from("a")), Ident(String::from("a")))
                )),
            }]
        };

        assert_eq!(optimize(rules), unrolled);
    }

    #[test]
    fn unroll_loop_max() {
        let rules = vec![Rule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: Expr::RepMax(Box::new(Expr::Str("a".to_owned())), 3),
        }];
        let unrolled = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(
                    Opt(Str(String::from("a"))),
                    Seq(Opt(Str(String::from("a"))), Opt(Str(String::from("a"))))
                )),
            }]
        };

        assert_eq!(optimize(rules), unrolled);
    }

    #[test]
    fn unroll_loop_min() {
        let rules = vec![Rule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: Expr::RepMin(Box::new(Expr::Str("a".to_owned())), 2),
        }];
        let unrolled = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(
                    Str(String::from("a")),
                    Seq(Str(String::from("a")), Rep(Str(String::from("a"))))
                )),
            }]
        };

        assert_eq!(optimize(rules), unrolled);
    }

    #[test]
    fn unroll_loop_min_max() {
        let rules = vec![Rule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: Expr::RepMinMax(Box::new(Expr::Str("a".to_owned())), 2, 3),
        }];
        let unrolled = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                /* TODO possible room for improvement here:
                 * if the sequences were rolled out in the opposite
                 * order, we could further optimize the strings
                 * in cases like this.
                Str(String::from(("aa")),
                Opt(Str(String::from("a")))
                */
                expr: box_tree!(Seq(
                    Str(String::from("a")),
                    Seq(Str(String::from("a")), Opt(Str(String::from("a"))))
                )),
            }]
        };

        assert_eq!(optimize(rules), unrolled);
    }

    #[test]
    fn concat_insensitive_strings() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(
                    Seq(Insens(String::from("a")), Insens(String::from("b"))),
                    Seq(Insens(String::from("c")), Insens(String::from("d")))
                )),
            }]
        };
        let concatenated = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Atomic,
            expr: OptimizedExpr::Insens(String::from("abcd")),
        }];

        assert_eq!(optimize(rules), concatenated);
    }

    #[test]
    fn long_common_sequence() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Choice(
                    Seq(
                        Ident(String::from("a")),
                        Seq(Ident(String::from("b")), Ident(String::from("c")))
                    ),
                    Seq(
                        Seq(Ident(String::from("a")), Ident(String::from("b"))),
                        Ident(String::from("d"))
                    )
                )),
            }]
        };
        let optimized = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Seq(
                    Ident(String::from("a")),
                    Seq(
                        Ident(String::from("b")),
                        Choice(Ident(String::from("c")), Ident(String::from("d")))
                    )
                )),
            }]
        };

        assert_eq!(optimize(rules), optimized);
    }

    #[test]
    fn short_common_sequence() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Choice(
                    Seq(Ident(String::from("a")), Ident(String::from("b"))),
                    Ident(String::from("a"))
                )),
            }]
        };
        let optimized = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Atomic,
                expr: box_tree!(Seq(Ident(String::from("a")), Opt(Ident(String::from("b"))))),
            }]
        };

        assert_eq!(optimize(rules), optimized);
    }

    #[test]
    fn impossible_common_sequence() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Choice(
                    Ident(String::from("a")),
                    Seq(Ident(String::from("a")), Ident(String::from("b")))
                )),
            }]
        };
        let optimized = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Ident(String::from("a"))),
            }]
        };

        assert_eq!(optimize(rules), optimized);
    }

    #[test]
    fn lister() {
        let rules = {
            use crate::ast::Expr::*;
            vec![Rule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Seq(
                    Rep(Seq(Ident(String::from("a")), Ident(String::from("b")))),
                    Ident(String::from("a"))
                )),
            }]
        };
        let optimized = {
            use crate::optimizer::OptimizedExpr::*;
            vec![OptimizedRule {
                name: "rule".to_owned(),
                ty: RuleType::Silent,
                expr: box_tree!(Seq(
                    Ident(String::from("a")),
                    Rep(Seq(Ident(String::from("b")), Ident(String::from("a"))))
                )),
            }]
        };

        assert_eq!(optimize(rules), optimized);
    }

    mod display {
        use super::super::*;
        /// In previous implementation of Display for OptimizedExpr
        /// in commit 48e0a8bd3d43a17c1c78f099610b745d18ec0c5f (actually committed by me),
        /// Str("\n") will be displayed as
        /// "
        /// "
        ///
        /// It will not break the compilation in normal use.
        ///
        /// But when I use it in automatically generating documents,
        /// it will quite confusing and we'll be unable to distinguish \n and \r.
        ///
        /// And `cargo expand` will emit codes that can't be compiled,
        /// for it expand `#[doc("...")]` to `/// ...`,
        /// and when the document comment breaks the line,
        /// it will be expanded into wrong codes.
        #[test]
        fn control_character() {
            assert_eq!(OptimizedExpr::Str("\n".to_owned()).to_string(), "\"\\n\"");
            assert_eq!(
                OptimizedExpr::Insens("\n".to_owned()).to_string(),
                "^\"\\n\"",
            );
            assert_eq!(
                OptimizedExpr::Range("\n".to_owned(), "\r".to_owned()).to_string(),
                "('\\n'..'\\r')",
            );
            assert_eq!(
                OptimizedExpr::Skip(vec![
                    "\n".to_owned(),
                    "\r".to_owned(),
                    "\n\r".to_owned(),
                    "\0".to_owned(),
                ])
                .to_string(),
                r#"(!("\n" | "\r" | "\n\r" | "\0") ~ ANY)*"#,
            );

            assert_ne!(OptimizedExpr::Str("\n".to_owned()).to_string(), "\"\n\"");
        }

        #[test]
        fn str() {
            assert_eq!(OptimizedExpr::Str("a".to_owned()).to_string(), r#""a""#);
        }

        #[test]
        fn insens() {
            assert_eq!(OptimizedExpr::Insens("a".to_owned()).to_string(), r#"^"a""#);
        }

        #[test]
        fn range() {
            assert_eq!(
                OptimizedExpr::Range("a".to_owned(), "z".to_owned()).to_string(),
                r#"('a'..'z')"#,
            );
        }

        #[test]
        fn ident() {
            assert_eq!(OptimizedExpr::Ident("a".to_owned()).to_string(), r#"a"#);
        }

        #[test]
        fn peek_slice() {
            assert_eq!(OptimizedExpr::PeekSlice(0, None).to_string(), "PEEK[0..]");
            assert_eq!(
                OptimizedExpr::PeekSlice(0, Some(-1)).to_string(),
                "PEEK[0..-1]",
            );
            assert_eq!(
                OptimizedExpr::PeekSlice(2, Some(3)).to_string(),
                "PEEK[2..3]",
            );
            assert_eq!(
                OptimizedExpr::PeekSlice(2, Some(-1)).to_string(),
                "PEEK[2..-1]",
            );
            assert_eq!(OptimizedExpr::PeekSlice(0, None).to_string(), "PEEK[0..]");
        }

        #[test]
        fn pos_pred() {
            assert_eq!(
                OptimizedExpr::PosPred(Box::new(OptimizedExpr::NegPred(Box::new(
                    OptimizedExpr::Ident("a".to_owned()),
                ))))
                .to_string(),
                "&!a",
            );
            assert_eq!(
                OptimizedExpr::PosPred(Box::new(OptimizedExpr::Choice(
                    Box::new(OptimizedExpr::Rep(Box::new(OptimizedExpr::Ident(
                        "a".to_owned(),
                    )))),
                    Box::new(OptimizedExpr::Str("a".to_owned())),
                )))
                .to_string(),
                r#"&(a* | "a")"#,
            );
            assert_eq!(
                OptimizedExpr::PosPred(Box::new(OptimizedExpr::RestoreOnErr(Box::new(
                    OptimizedExpr::NegPred(Box::new(OptimizedExpr::Ident("a".to_owned()))),
                ))))
                .to_string(),
                "&!a",
            );
        }

        #[test]
        fn neg_pred() {
            assert_eq!(
                OptimizedExpr::NegPred(Box::new(OptimizedExpr::Ident("e".to_owned()))).to_string(),
                r#"!e"#,
            );
            assert_eq!(
                OptimizedExpr::NegPred(Box::new(OptimizedExpr::Choice(
                    Box::new(OptimizedExpr::Push(Box::new(OptimizedExpr::Ident(
                        "a".to_owned(),
                    )))),
                    Box::new(OptimizedExpr::Str("a".to_owned())),
                )))
                .to_string(),
                r#"!(PUSH(a) | "a")"#,
            );
        }

        #[test]
        fn seq() {
            assert_eq!(
                OptimizedExpr::Seq(
                    Box::new(OptimizedExpr::Ident("e1".to_owned())),
                    Box::new(OptimizedExpr::Ident("e2".to_owned())),
                )
                .to_string(),
                r#"(e1 ~ e2)"#,
            );
            assert_eq!(
                Expr::Seq(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Seq(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Ident("e3".to_owned())),
                    )),
                )
                .to_string(),
                "(e1 ~ e2 ~ e3)",
            );
            assert_eq!(
                Expr::Seq(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Seq(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Seq(
                            Box::new(Expr::Ident("e3".to_owned())),
                            Box::new(Expr::Ident("e4".to_owned())),
                        )),
                    )),
                )
                .to_string(),
                "(e1 ~ e2 ~ e3 ~ e4)",
            );
            assert_eq!(
                Expr::Seq(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Choice(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Seq(
                            Box::new(Expr::Ident("e3".to_owned())),
                            Box::new(Expr::Ident("e4".to_owned())),
                        )),
                    )),
                )
                .to_string(),
                "(e1 ~ (e2 | (e3 ~ e4)))",
            );
            assert_eq!(
                Expr::Seq(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Seq(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Choice(
                            Box::new(Expr::Ident("e3".to_owned())),
                            Box::new(Expr::Ident("e4".to_owned())),
                        )),
                    )),
                )
                .to_string(),
                "(e1 ~ e2 ~ (e3 | e4))",
            );
            assert_eq!(
                OptimizedExpr::Seq(
                    Box::new(OptimizedExpr::Rep(Box::new(OptimizedExpr::Str(
                        "a".to_owned(),
                    )))),
                    Box::new(OptimizedExpr::Seq(
                        Box::new(OptimizedExpr::Ident("b".to_owned())),
                        Box::new(OptimizedExpr::Insens("c".to_owned())),
                    )),
                )
                .to_string(),
                r#"("a"* ~ b ~ ^"c")"#,
            );
            assert_eq!(
                OptimizedExpr::Seq(
                    Box::new(OptimizedExpr::PosPred(Box::new(OptimizedExpr::Range(
                        "a".to_owned(),
                        "z".to_owned(),
                    )))),
                    Box::new(OptimizedExpr::NegPred(Box::new(OptimizedExpr::Opt(
                        Box::new(OptimizedExpr::Range("A".to_owned(), "Z".to_owned())),
                    )))),
                )
                .to_string(),
                "(&('a'..'z') ~ !('A'..'Z')?)",
            );
        }

        #[test]
        fn choice() {
            assert_eq!(
                OptimizedExpr::Choice(
                    Box::new(OptimizedExpr::Ident("e1".to_owned())),
                    Box::new(OptimizedExpr::Ident("e2".to_owned())),
                )
                .to_string(),
                r#"(e1 | e2)"#,
            );
            assert_eq!(
                Expr::Choice(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Choice(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Ident("e3".to_owned())),
                    )),
                )
                .to_string(),
                "(e1 | e2 | e3)",
            );
            assert_eq!(
                Expr::Choice(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Choice(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Choice(
                            Box::new(Expr::Ident("e3".to_owned())),
                            Box::new(Expr::Ident("e4".to_owned())),
                        )),
                    )),
                )
                .to_string(),
                "(e1 | e2 | e3 | e4)",
            );
            assert_eq!(
                Expr::Choice(
                    Box::new(Expr::Ident("e1".to_owned())),
                    Box::new(Expr::Seq(
                        Box::new(Expr::Ident("e2".to_owned())),
                        Box::new(Expr::Choice(
                            Box::new(Expr::Ident("e3".to_owned())),
                            Box::new(Expr::Ident("e4".to_owned())),
                        )),
                    )),
                )
                .to_string(),
                "(e1 | (e2 ~ (e3 | e4)))",
            );
            assert_eq!(
                OptimizedExpr::Choice(
                    Box::new(OptimizedExpr::Str("a".to_owned())),
                    Box::new(OptimizedExpr::Choice(
                        Box::new(OptimizedExpr::Push(Box::new(OptimizedExpr::Ident(
                            "b".to_owned(),
                        )))),
                        Box::new(OptimizedExpr::Insens("c".to_owned())),
                    )),
                )
                .to_string(),
                r#"("a" | PUSH(b) | ^"c")"#,
            );
        }

        #[test]
        fn opt() {
            assert_eq!(
                OptimizedExpr::Opt(Box::new(OptimizedExpr::Ident("e".to_owned()))).to_string(),
                "e?",
            );
        }

        #[test]
        fn rep() {
            assert_eq!(
                OptimizedExpr::Rep(Box::new(OptimizedExpr::Ident("x".to_owned()))).to_string(),
                "x*",
            );
            assert_eq!(
                OptimizedExpr::Rep(Box::new(OptimizedExpr::Range(
                    "0".to_owned(),
                    "9".to_owned(),
                )))
                .to_string(),
                "('0'..'9')*",
            );
        }

        #[test]
        #[cfg(feature = "grammar-extras")]
        fn rep_once() {
            assert_eq!(
                OptimizedExpr::RepOnce(Box::new(OptimizedExpr::Ident("e".to_owned()))).to_string(),
                "e+",
            );
            assert_eq!(
                OptimizedExpr::RepOnce(Box::new(OptimizedExpr::Range(
                    "0".to_owned(),
                    "9".to_owned(),
                )))
                .to_string(),
                "('0'..'9')+",
            );
        }

        #[test]
        fn skip() {
            assert_eq!(
                OptimizedExpr::Skip(
                    ["a", "bc"]
                        .into_iter()
                        .map(|s| s.to_owned())
                        .collect::<Vec<_>>(),
                )
                .to_string(),
                r#"(!("a" | "bc") ~ ANY)*"#,
            );
        }

        #[test]
        fn inline_skip() {
            use crate::ast::Expr::*;
            let rules = vec![
                Rule {
                    name: "inline".to_owned(),
                    ty: RuleType::Atomic,
                    expr: Str("a".to_owned()),
                },
                Rule {
                    name: "skip".to_owned(),
                    ty: RuleType::Atomic,
                    expr: box_tree!(Rep(Seq(
                        NegPred(Choice(
                            Ident(String::from("inline")),
                            Str(String::from("b"))
                        )),
                        Ident("ANY".to_owned())
                    ))),
                },
            ];
            let map = to_hash_map(&rules);
            let rule = skipper::skip(rules[1].clone(), &map);
            assert!(matches!(rule, Rule { expr: Skip(..), .. }));
            let choices = match rule.expr {
                Skip(choices) => choices,
                _ => unreachable!(),
            };
            assert_eq!(choices, vec!["a".to_owned(), "b".to_owned()]);
        }

        #[test]
        fn push() {
            assert_eq!(
                OptimizedExpr::Push(Box::new(OptimizedExpr::Ident("e".to_owned()))).to_string(),
                "PUSH(e)",
            );
        }

        #[test]
        #[cfg(feature = "grammar-extras")]
        fn push_literal() {
            assert_eq!(
                OptimizedExpr::PushLiteral("a \" b".to_owned()).to_string(),
                r#"PUSH_LITERAL("a \" b")"#,
            );
        }

        #[test]
        #[cfg(feature = "grammar-extras")]
        fn node_tag() {
            assert_eq!(
                OptimizedExpr::NodeTag(
                    Box::new(OptimizedExpr::Ident("expr".to_owned())),
                    "label".to_owned(),
                )
                .to_string(),
                r#"(#label = expr)"#,
            );
            assert_eq!(
                OptimizedExpr::NodeTag(
                    Box::new(OptimizedExpr::Ident("x".to_owned())),
                    "X".to_owned(),
                )
                .to_string(),
                r#"(#X = x)"#,
            );
            assert_eq!(
                OptimizedExpr::NodeTag(
                    Box::new(OptimizedExpr::Seq(
                        Box::new(OptimizedExpr::Ident("x".to_owned())),
                        Box::new(OptimizedExpr::Str("y".to_owned())),
                    )),
                    "X".to_owned(),
                )
                .to_string(),
                r#"(#X = (x ~ "y"))"#,
            );
        }

        #[test]
        fn restore_on_err() {
            assert_eq!(
                OptimizedExpr::RestoreOnErr(Box::new(OptimizedExpr::Ident("e".to_owned())))
                    .to_string(),
                "e",
            );
        }
    }
}
