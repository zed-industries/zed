// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, ExprClosure, Token,
};

use crate::{
    clone::{Capture, CaptureKind, UpgradeBehaviour},
    utils::crate_ident_new,
};

struct Closure {
    captures: Vec<Capture>,
    args: Vec<Ident>,
    upgrade_behaviour: UpgradeBehaviour,
    closure: ExprClosure,
    constructor: &'static str,
}

impl Parse for Closure {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Err(syn::Error::new(Span::call_site(), "expected a closure"));
        }

        let mut captures: Vec<Capture> = vec![];
        let mut upgrade_behaviour: Option<(UpgradeBehaviour, Span)> = None;

        loop {
            // There must either be one or no attributes here. Multiple attributes are not
            // supported.
            //
            // If this is a capture attribute, it must be followed by an identifier.
            // If this is an upgrade failure attribute, it might be followed by a closure. After the
            // upgrade failure attribute there must not be any further attributes.
            //
            // If this is not an attribute then it is a closure, async closure or async block which
            // is handled outside the loop
            let attrs = input.call(Attribute::parse_outer)?;
            if attrs.is_empty() {
                break;
            };

            if let Some(capture) = Capture::maybe_parse(&attrs, input)? {
                if capture.kind == CaptureKind::Watch
                    && captures.iter().any(|c| c.kind == CaptureKind::Watch)
                {
                    return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "only one `watch` capture is allowed per closure",
                    ));
                }

                captures.push(capture);
            } else if let Some(behaviour) = UpgradeBehaviour::maybe_parse(&attrs, input)? {
                if upgrade_behaviour.is_some() {
                    return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "multiple upgrade failure attributes are not supported",
                    ));
                }

                upgrade_behaviour = Some((behaviour, attrs[0].span()));
                break;
            } else if let Some(ident) = attrs[0].path().get_ident() {
                return Err(syn::Error::new_spanned(
                        &attrs[0],
                        format!(
                            "unsupported attribute `{ident}`: only `watch`, `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported",
                        ),
                ));
            } else {
                return Err(syn::Error::new_spanned(
                        &attrs[0],
                        "unsupported attribute: only `strong`, `weak`, `weak_allow_none`, `to_owned`, `upgrade_or_else`, `upgrade_or_default` and `upgrade_or_panic` are supported",
                ));
            }
        }

        if let Some((_, ref span)) = upgrade_behaviour {
            if captures.iter().all(|c| c.kind != CaptureKind::Weak) {
                return Err(syn::Error::new(
                    *span,
                    "upgrade failure attribute can only be used together with weak variable captures",
                ));
            }
        }

        let upgrade_behaviour = upgrade_behaviour.map(|x| x.0).unwrap_or_default();

        let mut closure = input.parse::<ExprClosure>()?;
        if closure.asyncness.is_some() {
            return Err(syn::Error::new_spanned(
                closure,
                "async closures not supported",
            ));
        }
        if !captures.is_empty() && closure.capture.is_none() {
            return Err(syn::Error::new_spanned(
                closure,
                "closures need to capture variables by move. Please add the `move` keyword",
            ));
        }
        closure.capture = None;

        let args = closure
            .inputs
            .iter()
            .enumerate()
            .map(|(i, _)| Ident::new(&format!("____value{i}"), Span::call_site()))
            .collect();

        // Trailing comma, if any
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Closure {
            captures,
            args,
            upgrade_behaviour,
            closure,
            constructor: "new",
        })
    }
}

impl ToTokens for Closure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ident = crate_ident_new();

        let closure_ident = Ident::new("____closure", Span::call_site());
        let values_ident = Ident::new("____values", Span::call_site());
        let upgrade_failure_closure_ident =
            Ident::new("____upgrade_failure_closure", Span::call_site());
        let upgrade_failure_closure_wrapped_ident =
            Ident::new("____upgrade_failure_closure_wrapped", Span::call_site());

        let outer_before = self
            .captures
            .iter()
            .map(|c| c.outer_before_tokens(&crate_ident));
        let inner_before = self.captures.iter().map(|c| {
            c.inner_before_tokens(
                &crate_ident,
                &self.upgrade_behaviour,
                &upgrade_failure_closure_wrapped_ident,
                Some(quote! {
                    return #crate_ident::closure::IntoClosureReturnValue::into_closure_return_value(());
                }),
            )
        });
        let outer_after = self
            .captures
            .iter()
            .map(|c| c.outer_after_tokens(&crate_ident, &closure_ident));

        let arg_values = self.args.iter().enumerate().map(|(index, arg)| {
            let err_msg = format!("Wrong type for argument {index}: {{:?}}");
            quote! {
                let #arg = ::core::result::Result::unwrap_or_else(
                    #crate_ident::Value::get(&#values_ident[#index]),
                    |e| panic!(#err_msg, e),
                );
            }
        });
        let arg_names = &self.args;
        let args_len = self.args.len();
        let closure = &self.closure;
        let constructor = Ident::new(self.constructor, Span::call_site());

        let upgrade_failure_closure = match self.upgrade_behaviour {
            UpgradeBehaviour::Default => Some(quote! {
                let #upgrade_failure_closure_ident = ::std::default::Default::default;
                let #upgrade_failure_closure_wrapped_ident = ||
                    #crate_ident::closure::IntoClosureReturnValue::into_closure_return_value(
                        (#upgrade_failure_closure_ident)()
                    );
            }),
            UpgradeBehaviour::Expression(ref expr) => Some(quote! {
                let #upgrade_failure_closure_ident = move || {
                    #expr
                };
                let #upgrade_failure_closure_wrapped_ident = ||
                    #crate_ident::closure::IntoClosureReturnValue::into_closure_return_value(
                        (#upgrade_failure_closure_ident)()
                    );
            }),
            UpgradeBehaviour::Closure(ref closure_2) => Some(quote! {
                    let #upgrade_failure_closure_ident = #closure_2;
                    let #upgrade_failure_closure_wrapped_ident = ||
                        #crate_ident::closure::IntoClosureReturnValue::into_closure_return_value(
                            (#upgrade_failure_closure_ident)()
                        );
            }),
            _ => None,
        };

        let assert_return_type = upgrade_failure_closure.is_some().then(|| {
            quote! {
                fn ____same<T>(_a: &T, _b: impl Fn() -> T) {}
                ____same(&____res, #upgrade_failure_closure_ident);
            }
        });

        tokens.extend(quote! {
            {
                let #closure_ident = {
                    #(#outer_before)*
                    #crate_ident::closure::RustClosure::#constructor(move |#values_ident| {
                        assert_eq!(
                            #values_ident.len(),
                            #args_len,
                            "Expected {} arguments but got {}",
                            #args_len,
                            #values_ident.len(),
                        );
                        #upgrade_failure_closure
                        #(#inner_before)*
                        #(#arg_values)*
                        #crate_ident::closure::IntoClosureReturnValue::into_closure_return_value({
                            let ____res = (#closure)(#(#arg_names),*);
                            #assert_return_type
                            ____res
                        })
                    }
                    )
                };
                #(#outer_after)*
                #closure_ident
            }
        });
    }
}

pub(crate) fn closure_inner(
    input: proc_macro::TokenStream,
    constructor: &'static str,
) -> proc_macro::TokenStream {
    let mut closure = syn::parse_macro_input!(input as Closure);
    closure.constructor = constructor;
    closure.into_token_stream().into()
}
