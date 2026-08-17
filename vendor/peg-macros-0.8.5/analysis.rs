use proc_macro2::Span;
use std::collections::HashMap;

use crate::ast::*;

pub struct GrammarAnalysis<'a> {
    pub rules: HashMap<String, &'a Rule>,
    pub left_recursion: Vec<LeftRecursionError>,
    pub loop_nullability: Vec<LoopNullabilityError>,
}

pub fn check<'a>(grammar: &'a Grammar) -> GrammarAnalysis<'a> {
    let mut rules = HashMap::new();

    // Pick only the first for duplicate rules (the duplicate is reported when translating the rule)
    for rule in grammar.iter_rules() {
        rules.entry(rule.name.to_string()).or_insert(rule);
    }

    let (rule_nullability, left_recursion) = LeftRecursionVisitor::check(grammar, &rules);
    let loop_nullability = LoopNullabilityVisitor::check(grammar, &rule_nullability);

    GrammarAnalysis {
        rules,
        left_recursion,
        loop_nullability,
    }
}

/// Check for infinite loops in the form of left recursion.
///
/// If a PEG expression recurses without first consuming input, it will
/// recurse until the stack overflows.
struct LeftRecursionVisitor<'a> {
    stack: Vec<String>,
    rules: &'a HashMap<String, &'a Rule>,
    errors: Vec<LeftRecursionError>,
}

pub struct LeftRecursionError {
    pub span: Span,
    pub path: Vec<String>,
}

impl LeftRecursionError {
    pub fn msg(&self) -> String {
        format!(
            "left recursive rules create an infinite loop: {}",
            self.path.join(" -> ")
        )
    }
}

impl<'a> LeftRecursionVisitor<'a> {
    fn check(grammar: &'a Grammar, rules: &HashMap<String, &'a Rule>) -> (HashMap<String, bool>, Vec<LeftRecursionError>) {
        let mut visitor = LeftRecursionVisitor {
            rules,
            errors: Vec::new(),
            stack: Vec::new(),
        };

        let mut rule_nullability: HashMap<String, bool> = HashMap::new();

        for rule in grammar.iter_rules() {
            let nullable = visitor.walk_rule(rule);
            debug_assert!(visitor.stack.is_empty());
            rule_nullability.entry(rule.name.to_string()).or_insert(nullable);
        }

        (rule_nullability, visitor.errors)
    }

    fn walk_rule(&mut self, rule: &'a Rule) -> bool {
        self.stack.push(rule.name.to_string());
        let res = self.walk_expr(&rule.expr);
        self.stack.pop().unwrap();
        res
    }

    /// Walk the prefix of an expression that can be reached without consuming
    /// input.
    ///
    /// Returns true if the rule is known to match completely without consuming
    /// any input. This is a conservative heuristic, if unknown, we return false
    /// to avoid reporting false-positives for left recursion.
    fn walk_expr(&mut self, this_expr: &SpannedExpr) -> bool {
        use self::Expr::*;
        match this_expr.expr {
            RuleExpr(ref rule_ident, _, _) => {
                let name = rule_ident.to_string();

                if let Some(rule) = self.rules.get(&name) {
                    if let Some(loop_start) = self
                        .stack
                        .iter()
                        .position(|caller_name| caller_name == &name)
                    {
                        let mut recursive_loop = self.stack[loop_start..].to_vec();
                        recursive_loop.push(name.clone());
                        match rule.cache {
                            None | Some(Cache::Simple) =>
                                self.errors.push(LeftRecursionError {
                                    path: recursive_loop,
                                    span: rule_ident.span(),
                                }),
                            _ => ()

                        }
                        return false;
                    }
                    self.walk_rule(rule)
                } else {
                    // Missing rule would have already been reported
                   false
                }
            }

            ActionExpr(ref elems, ..) => {
                for elem in elems {
                    if !self.walk_expr(&elem.expr) {
                        return false;
                    }
                }

                true
            }

            ChoiceExpr(ref choices) => {
                let mut nullable = false;
                for expr in choices {
                    nullable |= self.walk_expr(expr);
                }
                nullable
            }

            OptionalExpr(ref expr) | PosAssertExpr(ref expr) | NegAssertExpr(ref expr) => {
                self.walk_expr(expr);
                true
            }

            Repeat { ref inner, ref bound, .. } => {
                let inner_nullable = self.walk_expr(inner);
                inner_nullable | !bound.has_lower_bound()
            }

            MatchStrExpr(ref expr) | QuietExpr(ref expr) => self.walk_expr(expr),

            PrecedenceExpr { ref levels } => {
                let mut nullable = false;

                for level in levels {
                    for operator in &level.operators {
                        let mut operator_nullable = true;
                        for element in &operator.elements {
                            if !self.walk_expr(&element.expr) {
                                operator_nullable = false;
                                break;
                            }
                        }
                        nullable |= operator_nullable;
                    }
                }

               nullable
            }

            | LiteralExpr(_)
            | PatternExpr(_)
            | MethodExpr(_, _)
            | CustomExpr(_)
            | FailExpr(_)
            | MarkerExpr(_) => false,

            PositionExpr => true,
        }
    }
}

/// Check for loops whose body can succeed without consuming any input, which
/// will loop infinitely.
struct LoopNullabilityVisitor<'a> {
    rule_nullability: &'a HashMap<String, bool>,
    errors: Vec<LoopNullabilityError>,
}

pub struct LoopNullabilityError {
    pub span: Span,
}

impl LoopNullabilityError {
    pub fn msg(&self) -> String {
        format!("loops infinitely because loop body can match without consuming input")
    }
}


impl<'a> LoopNullabilityVisitor<'a> {
    fn check(grammar: &'a Grammar, rule_nullability: &HashMap<String, bool>) -> Vec<LoopNullabilityError> {
        let mut visitor = LoopNullabilityVisitor {
            rule_nullability,
            errors: Vec::new(),
        };

        for rule in grammar.iter_rules() {
            visitor.walk_expr(&rule.expr);
        }

        visitor.errors
    }


    /// Walk an expr and its children analyzing the nullability of loop bodies.
    ///
    /// Returns true if the rule is known to match completely without consuming
    /// any input. This is a conservative heuristic; if unknown, we return false
    /// to avoid reporting false-positives.
    ///
    /// This is very similar to LeftRecursionVisitor::walk_expr, but walks the
    /// entire expression tree rather than just the nullable prefix, and doesn't
    /// recurse into calls.
    fn walk_expr(&mut self, this_expr: &SpannedExpr) -> bool {
        use self::Expr::*;
        match this_expr.expr {
            RuleExpr(ref rule_ident, _, _) => {
                let name = rule_ident.to_string();
                *self.rule_nullability.get(&name).unwrap_or(&false)
            }

            ActionExpr(ref elems, ..) => {
                let mut nullable = true;
                for elem in elems {
                    nullable &= self.walk_expr(&elem.expr);
                }
                nullable
            }

            ChoiceExpr(ref choices) => {
                let mut nullable = false;
                for expr in choices {
                    nullable |= self.walk_expr(expr);
                }
                nullable
            }

            OptionalExpr(ref expr) | PosAssertExpr(ref expr) | NegAssertExpr(ref expr) => {
                self.walk_expr(expr);
                true
            }

            Repeat { ref inner, ref bound, ref sep } => {
                let inner_nullable = self.walk_expr(inner);
                let sep_nullable = sep.as_ref().map_or(true, |sep| self.walk_expr(sep));

                // The entire purpose of this analysis: report errors if the loop body is nullable
                if inner_nullable && sep_nullable && !bound.has_upper_bound() {
                    self.errors.push(LoopNullabilityError { span: this_expr.span });
                }

                inner_nullable | !bound.has_lower_bound()
            }

            MatchStrExpr(ref expr) | QuietExpr(ref expr) => self.walk_expr(expr),

            PrecedenceExpr { ref levels } => {
                let mut nullable = false;

                for level in levels {
                    for operator in &level.operators {
                        let mut operator_nullable = true;
                        for element in &operator.elements {
                            operator_nullable &= self.walk_expr(&element.expr);
                        }
                        nullable |= operator_nullable;
                    }
                }

                nullable
            }

            | LiteralExpr(_)
            | PatternExpr(_)
            | MethodExpr(_, _)
            | CustomExpr(_)
            | FailExpr(_)
            | MarkerExpr(_) => false,

            PositionExpr => true,
        }
    }
}
