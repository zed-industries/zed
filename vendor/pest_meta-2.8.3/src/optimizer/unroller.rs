// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::ast::*;

pub fn unroll(rule: Rule) -> Rule {
    let Rule { name, ty, expr } = rule;
    Rule {
        name,
        ty,
        expr: expr.map_bottom_up(|expr| match expr {
            #[cfg(not(feature = "grammar-extras"))]
            Expr::RepOnce(expr) => Expr::Seq(expr.clone(), Box::new(Expr::Rep(expr))),
            Expr::RepExact(expr, num) => (1..num + 1)
                .map(|_| *expr.clone())
                .rev()
                .fold(None, |rep, expr| match rep {
                    None => Some(expr),
                    Some(rep) => Some(Expr::Seq(Box::new(expr), Box::new(rep))),
                })
                .unwrap(),
            Expr::RepMin(expr, min) => (1..min + 2)
                .map(|i| {
                    if i <= min {
                        *expr.clone()
                    } else {
                        Expr::Rep(expr.clone())
                    }
                })
                .rev()
                .fold(None, |rep, expr| match rep {
                    None => Some(expr),
                    Some(rep) => Some(Expr::Seq(Box::new(expr), Box::new(rep))),
                })
                .unwrap(),
            Expr::RepMax(expr, max) => (1..max + 1)
                .map(|_| Expr::Opt(expr.clone()))
                .rev()
                .fold(None, |rep, expr| match rep {
                    None => Some(expr),
                    Some(rep) => Some(Expr::Seq(Box::new(expr), Box::new(rep))),
                })
                .unwrap(),
            Expr::RepMinMax(expr, min, max) => (1..max + 1)
                .map(|i| {
                    if i <= min {
                        *expr.clone()
                    } else {
                        Expr::Opt(expr.clone())
                    }
                })
                .rev()
                .fold(None, |rep, expr| match rep {
                    None => Some(expr),
                    Some(rep) => Some(Expr::Seq(Box::new(expr), Box::new(rep))),
                })
                .unwrap(),
            expr => expr,
        }),
    }
}
