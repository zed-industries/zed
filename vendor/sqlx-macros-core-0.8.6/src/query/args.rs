use crate::database::DatabaseExt;
use crate::query::QueryMacroInput;
use either::Either;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use sqlx_core::describe::Describe;
use syn::spanned::Spanned;
use syn::{Expr, ExprCast, ExprGroup, Type};

/// Returns a tokenstream which typechecks the arguments passed to the macro
/// and binds them to `DB::Arguments` with the ident `query_args`.
pub fn quote_args<DB: DatabaseExt>(
    input: &QueryMacroInput,
    info: &Describe<DB>,
) -> crate::Result<TokenStream> {
    let db_path = DB::db_path();

    if input.arg_exprs.is_empty() {
        return Ok(quote! {
            let query_args = ::core::result::Result::<_, ::sqlx::error::BoxDynError>::Ok(<#db_path as ::sqlx::database::Database>::Arguments::<'_>::default());
        });
    }

    let arg_names = (0..input.arg_exprs.len())
        .map(|i| format_ident!("arg{}", i))
        .collect::<Vec<_>>();

    let arg_name = &arg_names;
    let arg_expr = input.arg_exprs.iter().cloned().map(strip_wildcard);

    let arg_bindings = quote! {
        #(let #arg_name = &(#arg_expr);)*
    };

    let args_check = match info.parameters() {
        None | Some(Either::Right(_)) => {
            // all we can do is check arity which we did
            TokenStream::new()
        }

        Some(Either::Left(_)) if !input.checked => {
            // this is an `*_unchecked!()` macro invocation
            TokenStream::new()
        }

        Some(Either::Left(params)) => {
            params
                .iter()
                .zip(arg_names.iter().zip(&input.arg_exprs))
                .enumerate()
                .map(|(i, (param_ty, (name, expr)))| -> crate::Result<_> {
                    if get_type_override(expr).is_some() {
                        // cast will fail to compile if the type does not match
                        // and we strip casts to wildcard
                        return Ok(quote!());
                    }

                    let param_ty =
                        DB::param_type_for_id(param_ty)
                            .ok_or_else(|| {
                                if let Some(feature_gate) = DB::get_feature_gate(param_ty) {
                                    format!(
                                        "optional sqlx feature `{}` required for type {} of param #{}",
                                        feature_gate,
                                        param_ty,
                                        i + 1,
                                    )
                                } else {
                                    format!(
                                        "no built in mapping found for type {} for param #{}; \
                                        a type override may be required, see documentation for details",
                                        param_ty,
                                        i + 1
                                    )
                                }
                            })?
                            .parse::<TokenStream>()
                            .map_err(|_| format!("Rust type mapping for {param_ty} not parsable"))?;

                    Ok(quote_spanned!(expr.span() =>
                        // this shouldn't actually run
                        #[allow(clippy::missing_panics_doc, clippy::unreachable)]
                        if false {
                            use ::sqlx::ty_match::{WrapSameExt as _, MatchBorrowExt as _};

                            // evaluate the expression only once in case it contains moves
                            let expr = ::sqlx::ty_match::dupe_value(#name);

                            // if `expr` is `Option<T>`, get `Option<$ty>`, otherwise `$ty`
                            let ty_check = ::sqlx::ty_match::WrapSame::<#param_ty, _>::new(&expr).wrap_same();

                            // if `expr` is `&str`, convert `String` to `&str`
                            let (mut _ty_check, match_borrow) = ::sqlx::ty_match::MatchBorrow::new(ty_check, &expr);

                            _ty_check = match_borrow.match_borrow();

                            // this causes move-analysis to effectively ignore this block
                            ::std::unreachable!();
                        }
                    ))
                })
                .collect::<crate::Result<TokenStream>>()?
        }
    };

    let args_count = input.arg_exprs.len();

    Ok(quote! {
        #arg_bindings

        #args_check

        let mut query_args = <#db_path as ::sqlx::database::Database>::Arguments::<'_>::default();
        query_args.reserve(
            #args_count,
            0 #(+ ::sqlx::encode::Encode::<#db_path>::size_hint(#arg_name))*
        );
        let query_args = ::core::result::Result::<_, ::sqlx::error::BoxDynError>::Ok(query_args)
        #(.and_then(move |mut query_args| query_args.add(#arg_name).map(move |()| query_args) ))*;
    })
}

fn get_type_override(expr: &Expr) -> Option<&Type> {
    match expr {
        Expr::Group(group) => get_type_override(&group.expr),
        Expr::Cast(cast) => Some(&cast.ty),
        _ => None,
    }
}

fn strip_wildcard(expr: Expr) -> Expr {
    match expr {
        Expr::Group(ExprGroup {
            attrs,
            group_token,
            expr,
        }) => Expr::Group(ExprGroup {
            attrs,
            group_token,
            expr: Box::new(strip_wildcard(*expr)),
        }),
        // we want to retain casts if they semantically matter
        Expr::Cast(ExprCast {
            attrs,
            expr,
            as_token,
            ty,
        }) => match *ty {
            // cast to wildcard `_` will produce weird errors; we interpret it as taking the value as-is
            Type::Infer(_) => *expr,
            _ => Expr::Cast(ExprCast {
                attrs,
                expr,
                as_token,
                ty,
            }),
        },
        _ => expr,
    }
}
