// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.
use std::collections::HashMap;

use crate::optimizer::*;

pub fn restore_on_err(
    rule: OptimizedRule,
    rules: &HashMap<String, OptimizedExpr>,
) -> OptimizedRule {
    let OptimizedRule { name, ty, expr } = rule;
    let expr = expr.map_bottom_up(|expr| wrap_branching_exprs(expr, rules));
    OptimizedRule { name, ty, expr }
}

fn wrap_branching_exprs(
    expr: OptimizedExpr,
    rules: &HashMap<String, OptimizedExpr>,
) -> OptimizedExpr {
    match expr {
        OptimizedExpr::Opt(expr) => {
            if child_modifies_state(&expr, rules, &mut HashMap::new()) {
                OptimizedExpr::Opt(Box::new(OptimizedExpr::RestoreOnErr(expr)))
            } else {
                OptimizedExpr::Opt(expr)
            }
        }
        OptimizedExpr::Choice(lhs, rhs) => {
            let wrapped_lhs = if child_modifies_state(&lhs, rules, &mut HashMap::new()) {
                Box::new(OptimizedExpr::RestoreOnErr(lhs))
            } else {
                lhs
            };
            let wrapped_rhs = if child_modifies_state(&rhs, rules, &mut HashMap::new()) {
                Box::new(OptimizedExpr::RestoreOnErr(rhs))
            } else {
                rhs
            };
            OptimizedExpr::Choice(wrapped_lhs, wrapped_rhs)
        }
        OptimizedExpr::Rep(expr) => {
            if child_modifies_state(&expr, rules, &mut HashMap::new()) {
                OptimizedExpr::Rep(Box::new(OptimizedExpr::RestoreOnErr(expr)))
            } else {
                OptimizedExpr::Rep(expr)
            }
        }
        _ => expr,
    }
}

fn child_modifies_state(
    expr: &OptimizedExpr,
    rules: &HashMap<String, OptimizedExpr>,
    cache: &mut HashMap<String, Option<bool>>,
) -> bool {
    expr.iter_top_down().any(|expr| match expr {
        OptimizedExpr::Push(_) => true,
        OptimizedExpr::Ident(ref name) if name == "DROP" => true,
        OptimizedExpr::Ident(ref name) if name == "POP" => true,
        OptimizedExpr::Ident(ref name) => match cache.get(name).cloned() {
            Some(option) => match option {
                Some(cached) => cached,
                None => {
                    cache.insert(name.to_owned(), Some(false));
                    false
                }
            },
            None => {
                cache.insert(name.to_owned(), None);

                let result = match rules.get(name) {
                    Some(expr) => child_modifies_state(expr, rules, cache),
                    None => false,
                };

                cache.insert(name.to_owned(), Some(result));

                result
            }
        },
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimizer::OptimizedExpr::*;

    #[test]
    fn restore_no_stack_children() {
        let rules = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Normal,
            expr: box_tree!(Opt(Str("a".to_string()))),
        }];

        assert_eq!(
            restore_on_err(rules[0].clone(), &to_optimized_hash_map(&rules)),
            rules[0].clone()
        );
    }

    #[test]
    fn restore_with_child_stack_ops() {
        let rules = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Normal,
            expr: box_tree!(Rep(Push(Str("a".to_string())))),
        }];

        let restored = OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Normal,
            expr: box_tree!(Rep(RestoreOnErr(Push(Str("a".to_string()))))),
        };

        assert_eq!(
            restore_on_err(rules[0].clone(), &to_optimized_hash_map(&rules)),
            restored
        );
    }

    #[test]
    fn restore_choice_branch_with_and_branch_without() {
        let rules = vec![OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Normal,
            expr: box_tree!(Choice(Push(Str("a".to_string())), Str("a".to_string()))),
        }];

        let restored = OptimizedRule {
            name: "rule".to_owned(),
            ty: RuleType::Normal,
            expr: box_tree!(Choice(
                RestoreOnErr(Push(Str("a".to_string()))),
                Str("a".to_string())
            )),
        };

        assert_eq!(
            restore_on_err(rules[0].clone(), &to_optimized_hash_map(&rules)),
            restored
        );
    }
}
