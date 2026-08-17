// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::ast::*;

pub fn rotate(rule: Rule) -> Rule {
    fn rotate_internal(expr: Expr) -> Expr {
        match expr {
            Expr::Seq(lhs, rhs) => {
                let lhs = *lhs;
                match lhs {
                    Expr::Seq(ll, lr) => {
                        rotate_internal(Expr::Seq(ll, Box::new(Expr::Seq(lr, rhs))))
                    }
                    lhs => Expr::Seq(Box::new(lhs), rhs),
                }
            }
            Expr::Choice(lhs, rhs) => {
                let lhs = *lhs;
                match lhs {
                    Expr::Choice(ll, lr) => {
                        rotate_internal(Expr::Choice(ll, Box::new(Expr::Choice(lr, rhs))))
                    }
                    lhs => Expr::Choice(Box::new(lhs), rhs),
                }
            }
            expr => expr,
        }
    }

    let Rule { name, ty, expr } = rule;
    Rule {
        name,
        ty,
        expr: expr.map_top_down(rotate_internal),
    }
}
