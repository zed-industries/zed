pub(crate) mod map;
pub(crate) mod set;

use crate::util::prelude::*;
use std::collections::HashSet;

pub(crate) fn validate_expressions_are_unique<'k, I>(err_label: &str, items: I) -> Option<Error>
where
    I: IntoIterator<Item = &'k syn::Expr>,
{
    let mut errors = Error::accumulator();

    let mut exprs = HashSet::new();

    items
        .into_iter()
        .filter(|item| is_pure(item))
        .for_each(|new_item| {
            let existing = match exprs.replace(new_item) {
                Some(existing) => existing,
                _ => return,
            };
            errors.extend([
                err!(existing, "duplicate {err_label}"),
                err!(new_item, "duplicate {err_label}"),
            ]);
        });

    errors.finish().err()
}

fn is_pure(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Binary(binary) => is_pure(&binary.left) && is_pure(&binary.right),
        syn::Expr::Group(group) => is_pure(&group.expr),
        syn::Expr::Lit(_) => true,
        syn::Expr::Paren(paren) => is_pure(&paren.expr),
        syn::Expr::Unary(unary) => is_pure(&unary.expr),
        _ => false,
    }
}
