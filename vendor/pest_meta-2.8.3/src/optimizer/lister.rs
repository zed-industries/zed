// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::ast::*;

pub fn list(rule: Rule) -> Rule {
    let Rule { name, ty, expr } = rule;
    Rule {
        name,
        ty,
        expr: expr.map_bottom_up(|expr| {
            match expr {
                Expr::Seq(l, r) => match *l {
                    Expr::Rep(l) => {
                        let l = *l;
                        match l {
                            Expr::Seq(l1, l2) => {
                                // Converts `(rule ~ rest)* ~ rule` to `rule ~ (rest ~ rule)*`,
                                // avoiding matching the last `rule` twice.
                                if l1 == r {
                                    Expr::Seq(l1, Box::new(Expr::Rep(Box::new(Expr::Seq(l2, r)))))
                                } else {
                                    Expr::Seq(Box::new(Expr::Rep(Box::new(Expr::Seq(l1, l2)))), r)
                                }
                            }
                            expr => Expr::Seq(Box::new(Expr::Rep(Box::new(expr))), r),
                        }
                    }
                    expr => Expr::Seq(Box::new(expr), r),
                },
                expr => expr,
            }
        }),
    }
}
