use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{spanned::Spanned, FnArg, ItemFn, Meta};

use super::utils::is_strategy;

/// Validate an `ItemFn` for some basic sanity checks
///
/// Many checks are deferred to rustc (e.g. rustc already errors if you make a test function
/// unsafe, so we just transparently pass unsafe through to the generated function and let rustc
/// emit the error)
pub(super) fn validate(f: &mut ItemFn) -> Result<(), TokenStream> {
    all_args_non_self(f)?;
    validate_parameter_attrs(f)?;

    Ok(())
}

fn all_args_non_self(f: &mut ItemFn) -> Result<(), TokenStream> {
    let first_self_arg = f
        .sig
        .inputs
        .iter()
        .find(|arg| matches!(arg, FnArg::Receiver(_)));

    match first_self_arg {
        None => Ok(()),
        Some(arg) => err(arg, "`self` parameters are forbidden"),
    }
}

/// Make sure we only have `#[strategy = <expr>]` attributes on function parameters
fn validate_parameter_attrs(f: &mut ItemFn) -> Result<(), TokenStream> {
    let mut error = quote::quote! {};

    for param in &mut f.sig.inputs {
        let FnArg::Typed(pat_ty) = param else {
            unreachable!("should be impossible due to `all_args_non_self`");
        };

        // add error for any non-`strategy` error or inner attributes (i.e. `#![...]` )
        for attr in pat_ty.attrs.iter().filter(|a| !is_strategy(a)) {
            error.extend(quote_spanned! {
                attr.span() => compile_error!("only `#[strategy = <expr>]` attributes are allowed here");
            });
        }

        let mut first_strategy_seen = false;
        let mut final_attrs = Vec::with_capacity(pat_ty.attrs.len());
        let old_attrs = std::mem::take(&mut pat_ty.attrs);

        // every strategy attr should have the form `#[strategy = <expr>]`
        for attr in old_attrs.into_iter().filter(is_strategy) {
            match attr.meta {
                // a "good" strategy - if we see more than one, emit an error
                Meta::NameValue(_) => {
                    if first_strategy_seen {
                        let pat =
                            pat_ty.pat.clone().into_token_stream().to_string();
                        let message = format!(
                            "{pat} has duplicate `#[strategy = ...] attribute`"
                        );
                        error.extend(quote_spanned! {
                            attr.span() => compile_error!(#message);
                        });
                    } else {
                        final_attrs.push(attr);
                        first_strategy_seen = true;
                    }
                }
                _ => {
                    error.extend(quote_spanned! {
                        attr.meta.span() => compile_error!("`strategy` attributes must have the form `#[strategy = <expr>]`");
                    });
                    final_attrs.push(attr);
                }
            }
        }

        pat_ty.attrs = final_attrs;
    }

    if error.is_empty() {
        Ok(())
    } else {
        Err(error)
    }
}

/// Helper function to generate `compile_error!()` outputs
fn err(span: impl Spanned, s: &str) -> Result<(), TokenStream> {
    Err(quote_spanned! { span.span() => compile_error!(#s) })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn validate_fails_with_self_arg() {
        let invalids = [
            parse_quote! {fn foo(self) {}},
            parse_quote! {fn foo(&self) {}},
            parse_quote! {fn foo(&mut self) {}},
            parse_quote! {fn foo(self: Self) {}},
            parse_quote! {fn foo(self: &Self) {}},
            parse_quote! {fn foo(self: &mut Self) {}},
            parse_quote! {fn foo(self: Box<Self>) {}},
            parse_quote! {fn foo(self: Rc<Self>) {}},
            parse_quote! {fn foo(self: Arc<Self>) {}},
        ];

        for mut invalid in invalids {
            assert!(validate(&mut invalid).is_err());
        }
    }

    #[test]
    fn validate_fails_with_duplicate() {
        let mut function = parse_quote! {
            fn foo(#[strategy = 1] #[strategy = 2] x: i32) {}
        };

        let error = validate(&mut function).unwrap_err();
        assert!(error.to_string().contains("compile_error"));
    }
}
