use proc_macro2::Ident;
use syn::{punctuated::Punctuated, Error, LitStr, Token, WhereClause, WherePredicate};

pub fn add_bounds(bounds: &LitStr, where_clause: &mut WhereClause) -> Result<(), Error> {
    let clauses = bounds.parse_with(Punctuated::<WherePredicate, Token![,]>::parse_terminated)?;
    for clause in clauses {
        where_clause.predicates.push(clause);
    }
    Ok(())
}

pub fn strip_raw(ident: &Ident) -> String {
    let as_string = ident.to_string();
    as_string
        .strip_prefix("r#")
        .map(ToString::to_string)
        .unwrap_or(as_string)
}
