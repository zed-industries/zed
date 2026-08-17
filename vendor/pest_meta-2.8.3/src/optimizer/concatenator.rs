// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::ast::*;

pub fn concatenate(rule: Rule) -> Rule {
    let Rule { name, ty, expr } = rule;
    Rule {
        name,
        ty,
        expr: expr.map_bottom_up(|expr| {
            if ty == RuleType::Atomic {
                match expr {
                    Expr::Seq(lhs, rhs) => match (*lhs, *rhs) {
                        (Expr::Str(lhs), Expr::Str(rhs)) => Expr::Str(lhs + &rhs),
                        (Expr::Insens(lhs), Expr::Insens(rhs)) => Expr::Insens(lhs + &rhs),
                        (lhs, rhs) => Expr::Seq(Box::new(lhs), Box::new(rhs)),
                    },
                    expr => expr,
                }
            } else {
                expr
            }
        }),
    }
}
