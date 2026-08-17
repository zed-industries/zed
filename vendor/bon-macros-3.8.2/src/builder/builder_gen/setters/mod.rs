use super::member::{SetterClosure, WithConfig};
use super::{BuilderGenCtx, NamedMember};
use crate::parsing::ItemSigConfig;
use crate::util::prelude::*;
use std::iter;

pub(crate) struct SettersCtx<'a> {
    base: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

impl<'a> SettersCtx<'a> {
    pub(crate) fn new(base: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self { base, member }
    }

    pub(crate) fn setter_methods(&self) -> Result<TokenStream> {
        match SettersItems::new(self) {
            SettersItems::Required(item) => self.setter_for_required_member(item),
            SettersItems::Optional(setters) => self.setters_for_optional_member(setters),
        }
    }

    fn setter_for_required_member(&self, item: SetterItem) -> Result<TokenStream> {
        let inputs;
        let expr;

        let member_type = self.member.ty.norm.as_ref();

        if let Some(with) = &self.member.config.with {
            inputs = self.underlying_inputs_from_with(with)?;
            expr = self.member_expr_from_with(with);
        } else if self.member.config.into.is_present() {
            inputs = vec![(
                pat_ident("value"),
                syn::parse_quote!(impl Into<#member_type>),
            )];
            expr = quote!(Into::into(value));
        } else {
            inputs = vec![(pat_ident("value"), member_type.clone())];
            expr = quote!(value);
        }

        let body = SetterBody::SetMember {
            expr: quote!(::core::option::Option::Some(#expr)),
        };

        Ok(self.setter_method(Setter {
            item,
            imp: SetterImpl { inputs, body },
        }))
    }

    fn setters_for_optional_member(&self, items: OptionalSettersItems) -> Result<TokenStream> {
        if let Some(with) = &self.member.config.with {
            return self.setters_for_optional_member_having_with(with, items);
        }

        let underlying_ty = self.member.underlying_norm_ty();
        let underlying_ty: syn::Type = if self.member.config.into.is_present() {
            syn::parse_quote!(impl Into<#underlying_ty>)
        } else {
            underlying_ty.clone()
        };

        let some_fn = Setter {
            item: items.some_fn,
            imp: SetterImpl {
                inputs: vec![(pat_ident("value"), underlying_ty.clone())],
                body: SetterBody::Forward {
                    body: {
                        let option_fn_name = &items.option_fn.name;
                        quote! {
                            self.#option_fn_name(Some(value))
                        }
                    },
                },
            },
        };

        let option_fn = Setter {
            item: items.option_fn,
            imp: SetterImpl {
                inputs: vec![(
                    pat_ident("value"),
                    syn::parse_quote!(Option<#underlying_ty>),
                )],
                body: SetterBody::SetMember {
                    expr: if self.member.config.into.is_present() {
                        quote! {
                            Option::map(value, Into::into)
                        }
                    } else {
                        quote!(value)
                    },
                },
            },
        };

        Ok([self.setter_method(some_fn), self.setter_method(option_fn)].concat())
    }

    fn setters_for_optional_member_having_with(
        &self,
        with: &WithConfig,
        items: OptionalSettersItems,
    ) -> Result<TokenStream> {
        let inputs = self.underlying_inputs_from_with(with)?;

        let idents = inputs.iter().map(|(pat, _)| &pat.ident);

        // If the closure accepts just a single input avoid wrapping it
        // in a tuple in the `option_fn` setter.
        let tuple_if_many = |val: TokenStream| -> TokenStream {
            if inputs.len() == 1 {
                val
            } else {
                quote!((#val))
            }
        };

        let ident_maybe_tuple = tuple_if_many(quote!( #( #idents ),* ));

        let some_fn = Setter {
            item: items.some_fn,
            imp: SetterImpl {
                inputs: inputs.clone(),
                body: SetterBody::Forward {
                    body: {
                        let option_fn_name = &items.option_fn.name;
                        quote! {
                            self.#option_fn_name(Some(#ident_maybe_tuple))
                        }
                    },
                },
            },
        };

        let option_fn_impl = SetterImpl {
            inputs: {
                let input_types = inputs.iter().map(|(_, ty)| ty);
                let input_types = tuple_if_many(quote!(#( #input_types, )*));

                vec![(pat_ident("value"), syn::parse_quote!(Option<#input_types>))]
            },
            body: SetterBody::SetMember {
                expr: {
                    let expr = self.member_expr_from_with(with);
                    quote! {
                        // Not using `Option::map` here because the `#expr`
                        // can contain a `?` operator for a fallible operation.
                        match value {
                            Some(#ident_maybe_tuple) => Some(#expr),
                            None => None,
                        }
                    }
                },
            },
        };

        let option_fn = Setter {
            item: items.option_fn,
            imp: option_fn_impl,
        };

        Ok([self.setter_method(some_fn), self.setter_method(option_fn)].concat())
    }

    /// This method is reused between the setter for the required member and
    /// the `some_fn` setter for the optional member.
    ///
    /// We intentionally keep the name and signature of the setter method
    /// for an optional member that accepts the value under the option the
    /// same as the setter method for the required member to keep the API
    /// of the builder compatible when a required member becomes optional.
    /// To be able to explicitly pass an `Option` value to the setter method
    /// users need to use the `maybe_{member_ident}` method.
    fn underlying_inputs_from_with(
        &self,
        with: &WithConfig,
    ) -> Result<Vec<(syn::PatIdent, syn::Type)>> {
        let inputs = match with {
            WithConfig::Closure(closure) => closure
                .inputs
                .iter()
                .map(|input| (input.pat.clone(), (*input.ty).clone()))
                .collect(),
            WithConfig::Some(some) => {
                let input_ty = self
                    .member
                    .underlying_norm_ty()
                    .option_type_param()
                    .ok_or_else(|| {
                        if self.member.ty.norm.is_option() {
                            err!(
                                some,
                                "the underlying type of this member is not `Option`; \
                                by default, members of type `Option` are optional and their \
                                'underlying type' is the type under the `Option`; \
                                you might be missing #[builder(required)]` annotation \
                                for this member"
                            )
                        } else {
                            err!(
                                &self.member.underlying_norm_ty(),
                                "`with = Some` only works for members with the underlying \
                                    type of `Option`;"
                            )
                        }
                    })?;

                vec![(pat_ident("value"), input_ty.clone())]
            }
            WithConfig::FromIter(from_iter) => {
                let collection_ty = self.member.underlying_norm_ty();

                let well_known_single_arg_suffixes = ["Vec", "Set", "Deque", "Heap", "List"];

                let err = || {
                    let mut from_iter_path = quote!(#from_iter).to_string();
                    from_iter_path.retain(|c| !c.is_whitespace());

                    err!(
                        collection_ty,
                        "the underlying type of this member is not a known collection type; \
                        only a collection type that matches the following patterns will be \
                        accepted by `#[builder(with = {from_iter_path})], where * at \
                        the beginning means the collection type may start with any prefix:\n\
                        - *Map<K, V>\n\
                        {}",
                        well_known_single_arg_suffixes
                            .iter()
                            .map(|suffix| { format!("- *{suffix}<T>") })
                            .join("\n")
                    )
                };

                let path = collection_ty.as_path_no_qself().ok_or_else(err)?;

                let last_segment = path.segments.last().ok_or_else(err)?;
                let args = match &last_segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => &args.args,
                    _ => return Err(err()),
                };

                let last_segment_ident_str = last_segment.ident.to_string();

                let item_ty = if well_known_single_arg_suffixes
                    .iter()
                    .any(|suffix| last_segment_ident_str.ends_with(suffix))
                {
                    // We don't compare for `len == 1` because there may be an optional last
                    // type argument for the allocator
                    if args.is_empty() {
                        return Err(err());
                    }

                    let arg = args.first().ok_or_else(err)?;

                    quote!(#arg)
                } else if last_segment_ident_str.ends_with("Map") {
                    // We don't compare for `len == 2` because there may be an optional last
                    // type argument for the allocator
                    if args.len() < 2 {
                        return Err(err());
                    }

                    let mut args = args.iter();
                    let key = args.next().ok_or_else(err)?;
                    let value = args.next().ok_or_else(err)?;

                    quote!((#key, #value))
                } else {
                    return Err(err());
                };

                vec![(
                    pat_ident("iter"),
                    syn::parse_quote!(impl IntoIterator<Item = #item_ty>),
                )]
            }
        };

        Ok(inputs)
    }

    fn member_expr_from_with(&self, with: &WithConfig) -> TokenStream {
        match with {
            WithConfig::Closure(closure) => self.member_expr_from_with_closure(with, closure),
            WithConfig::Some(some) => quote!(#some(value)),
            WithConfig::FromIter(from_iter) => quote!(#from_iter(iter)),
        }
    }

    fn member_expr_from_with_closure(
        &self,
        with: &WithConfig,
        closure: &SetterClosure,
    ) -> TokenStream {
        let body = &closure.body;

        let ty = self.member.underlying_norm_ty().to_token_stream();

        let output = Self::maybe_wrap_in_result(with, ty);

        // Closures aren't supported in `const` contexts at the time of this writing
        // (Rust 1.86.0), so we don't wrap it in a closure but we require the expression
        // to be simple so that it doesn't break out of the surrounding scope.
        // Search for `require_embeddable_const_expr` for more.
        if self.base.const_.is_some() {
            let body = quote! {{
                let value: #output = #body;
                value
            }};

            if closure.output.is_none() {
                return body;
            }

            return quote! {
                match #body {
                    Ok(value) => value,
                    Err(err) => return Err(err),
                }
            };
        }

        // Avoid wrapping the body in a block if it's already a block.
        let body = if matches!(body.as_ref(), syn::Expr::Block(_)) {
            body.to_token_stream()
        } else {
            quote!({ #body })
        };

        let question_mark = closure
            .output
            .is_some()
            .then(|| syn::Token![?](Span::call_site()));

        quote! {
            (move || -> #output #body)() #question_mark
        }
    }

    fn maybe_wrap_in_result(with: &WithConfig, ty: TokenStream) -> TokenStream {
        let closure = match with {
            WithConfig::Closure(closure) => closure,
            _ => return ty,
        };

        let output = match closure.output.as_ref() {
            Some(output) => output,
            None => return ty,
        };
        let result_path = &output.result_path;
        let err_ty = output.err_ty.iter();
        quote! {
            #result_path< #ty #(, #err_ty )* >
        }
    }

    fn setter_method(&self, setter: Setter) -> TokenStream {
        let Setter { item, imp } = setter;

        let maybe_mut = match imp.body {
            SetterBody::Forward { .. } => None,
            SetterBody::SetMember { .. } => Some(syn::Token![mut](Span::call_site())),
        };

        let body = match imp.body {
            SetterBody::Forward { body } => body,
            SetterBody::SetMember { expr } => {
                let mut output = if !self.member.is_stateful() {
                    quote! {
                        self
                    }
                } else {
                    let builder_ident = &self.base.builder_type.ident;

                    let maybe_receiver_field = self.base.receiver().map(|receiver| {
                        let ident = &receiver.field_ident;
                        quote!(#ident: self.#ident,)
                    });

                    let start_fn_args_fields_idents =
                        self.base.start_fn_args().map(|member| &member.ident);

                    let custom_fields_idents = self.base.custom_fields().map(|field| &field.ident);

                    quote! {
                        #builder_ident {
                            __unsafe_private_phantom: ::core::marker::PhantomData,
                            #( #custom_fields_idents: self.#custom_fields_idents, )*
                            #maybe_receiver_field
                            #( #start_fn_args_fields_idents: self.#start_fn_args_fields_idents, )*
                            __unsafe_private_named: self.__unsafe_private_named,
                        }
                    }
                };

                let result_output = self
                    .member
                    .config
                    .with
                    .as_ref()
                    .and_then(|with| with.as_closure()?.output.as_ref());

                if let Some(result_output) = result_output {
                    let result_path = &result_output.result_path;
                    output = quote!(#result_path::Ok(#output));
                }

                let index = &self.member.index;
                quote! {
                    self.__unsafe_private_named.#index = #expr;
                    #output
                }
            }
        };

        let state_mod = &self.base.state_mod.ident;

        let mut return_type = if !self.member.is_stateful() {
            quote! { Self }
        } else {
            let state_transition = format_ident!("Set{}", self.member.name.pascal_str);
            let builder_ident = &self.base.builder_type.ident;
            let generic_args = &self.base.generics.args;
            let state_var = &self.base.state_var;

            quote! {
                #builder_ident<#(#generic_args,)* #state_mod::#state_transition<#state_var>>
            }
        };

        if let Some(with) = &self.member.config.with {
            return_type = Self::maybe_wrap_in_result(with, return_type);
        }

        let where_clause = (!self.member.config.overwritable.is_present()).then(|| {
            let state_var = &self.base.state_var;
            let member_pascal = &self.member.name.pascal;
            quote! {
                where #state_var::#member_pascal: #state_mod::IsUnset,
            }
        });

        let SetterItem { name, vis, docs } = item;
        let pats = imp.inputs.iter().map(|(pat, _)| pat);
        let types = imp.inputs.iter().map(|(_, ty)| ty);
        let const_ = &self.base.const_;
        let fn_modifiers = self.member.respan(quote!(#vis #const_));

        // It's important to keep the span of `self` the same across all
        // references to it. Otherwise `self`s that have different spans will
        // be treated as totally different symbols due to the hygiene rules.
        let self_ = quote!(self);

        quote_spanned! {self.member.span=>
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We don't want to avoid using `impl Trait` in the setter. This way
                // the setter signature is easier to read, and anyway if you want to
                // specify a type hint for the method that accepts an `impl Into`, then
                // your design of this setter already went wrong.
                clippy::impl_trait_in_params,
                clippy::missing_const_for_fn,
                // When having a field which has one of the prefixes listed by
                // `clippy::wrong_self_convention` you will end up getting said lint
                // warning in your `bon::Builder` because we take self by value.
                clippy::wrong_self_convention,
            )]
            #[inline(always)]
            #(#fn_modifiers)* fn #name(#maybe_mut #self_, #( #pats: #types ),*) -> #return_type
            #where_clause
            {
                #body
            }
        }
    }
}

struct Setter {
    item: SetterItem,
    imp: SetterImpl,
}

struct SetterImpl {
    inputs: Vec<(syn::PatIdent, syn::Type)>,
    body: SetterBody,
}

enum SetterBody {
    /// The setter forwards the call to another method.
    Forward { body: TokenStream },

    /// The setter sets the member as usual and transitions the builder state.
    SetMember { expr: TokenStream },
}

enum SettersItems {
    Required(SetterItem),
    Optional(OptionalSettersItems),
}

struct OptionalSettersItems {
    some_fn: SetterItem,
    option_fn: SetterItem,
}

struct SetterItem {
    name: syn::Ident,
    vis: syn::Visibility,
    docs: Vec<syn::Attribute>,
}

impl SettersItems {
    fn new(ctx: &SettersCtx<'_>) -> Self {
        let SettersCtx { member, base } = ctx;
        let builder_type = &base.builder_type;

        let config = member.config.setters.as_ref();

        let common_name = config.and_then(|config| config.name.as_deref());
        let common_vis = config.and_then(|config| config.vis.as_deref());
        let common_docs =
            config.and_then(|config| config.doc.content.as_deref().map(Vec::as_slice));

        let doc = |docs: &str| iter::once(syn::parse_quote!(#[doc = #docs]));

        if member.is_required() {
            let docs = common_docs.unwrap_or(&member.docs);

            let header = "_**Required.**_\n\n";

            let docs = doc(header).chain(docs.iter().cloned()).collect();

            return Self::Required(SetterItem {
                name: common_name.unwrap_or(&member.name.snake).clone(),
                vis: common_vis.unwrap_or(&builder_type.vis).clone(),
                docs,
            });
        }

        let some_fn = config.and_then(|config| config.fns.some_fn.as_deref());
        let some_fn_name = some_fn
            .and_then(ItemSigConfig::name)
            .or(common_name)
            .unwrap_or(&member.name.snake)
            .clone();

        let option_fn = config.and_then(|config| config.fns.option_fn.as_deref());
        let option_fn_name = option_fn
            .and_then(ItemSigConfig::name)
            .cloned()
            .unwrap_or_else(|| {
                let base_name = common_name.unwrap_or(&member.name.snake);
                // It's important to preserve the original identifier span
                // to make IDE's "go to definition" work correctly. It's so
                // important that this doesn't use `format_ident!`, but rather
                // `syn::Ident::new` to set the span of the `Ident` explicitly.
                syn::Ident::new(&format!("maybe_{}", base_name.raw_name()), base_name.span())
            });

        let default = member.config.default.as_deref().and_then(|default| {
            if let Some(setters) = &member.config.setters {
                if let Some(default) = &setters.doc.default {
                    if default.skip.is_present() {
                        return None;
                    }
                }
            }

            let default = default
                .clone()
                .or_else(|| well_known_default(&member.ty.norm))
                .unwrap_or_else(|| {
                    let ty = &member.ty.norm;
                    syn::parse_quote!(<#ty as Default>::default())
                });

            let file = syn::parse_quote!(const _: () = #default;);
            let file = prettyplease::unparse(&file);

            let begin = file.find('=')?;
            let default = file.get(begin + 1..)?.trim();
            let default = default.strip_suffix(';')?;

            Some(default.to_owned())
        });

        let default = default.as_deref();

        // FIXME: the docs shouldn't reference the companion setter if that
        // setter has a lower visibility.
        let some_fn_docs = some_fn
            .and_then(ItemSigConfig::docs)
            .or(common_docs)
            .unwrap_or(&member.docs);

        let some_fn_docs =
            optional_setter_docs(default, &some_fn_name, &option_fn_name, some_fn_docs);

        let option_fn_docs = option_fn
            .and_then(ItemSigConfig::docs)
            .or(common_docs)
            .unwrap_or(&member.docs);

        let option_fn_docs =
            optional_setter_docs(default, &some_fn_name, &option_fn_name, option_fn_docs);

        let some_fn = SetterItem {
            name: some_fn_name,
            vis: some_fn
                .and_then(ItemSigConfig::vis)
                .or(common_vis)
                .unwrap_or(&builder_type.vis)
                .clone(),

            docs: some_fn_docs,
        };

        let option_fn = config.and_then(|config| config.fns.option_fn.as_deref());
        let option_fn = SetterItem {
            name: option_fn_name,

            vis: option_fn
                .and_then(ItemSigConfig::vis)
                .or(common_vis)
                .unwrap_or(&builder_type.vis)
                .clone(),

            docs: option_fn_docs,
        };

        Self::Optional(OptionalSettersItems { some_fn, option_fn })
    }
}

fn optional_setter_docs(
    default: Option<&str>,
    some_fn: &syn::Ident,
    option_fn: &syn::Ident,
    doc_comments: &[syn::Attribute],
) -> Vec<syn::Attribute> {
    let header = format!(
        "_**Optional** ([Some](Self::{some_fn}()) / [Option](Self::{option_fn}()) setters)._"
    );

    let mut attrs = vec![syn::parse_quote!(#[doc = #header])];

    if let Some(default) = default {
        let sep = if doc_comments.is_empty() { "" } else { "\n\n" };

        if default.contains('\n') || default.len() > 80 {
            // `no_doctest` helps to avoid interpreting the code block as
            // an "ignored but still runnable" doc test. See details:
            // - bon issue: https://github.com/elastio/bon/issues/359
            // - rust issue: https://github.com/rust-lang/rust/issues/63193
            let tail = format!("\n{default}\n````{sep}");
            attrs.extend([
                syn::parse_quote!(#[doc = " _**Default:**_\n"]),
                syn::parse_quote!(#[cfg_attr(doctest, doc = "````no_doctest")]),
                syn::parse_quote!(#[cfg_attr(not(doctest), doc = "````")]),
                syn::parse_quote!(#[doc = #tail]),
            ]);
        } else {
            let doc = format!(" _**Default:**_ ```{default}```.{sep}");
            attrs.push(syn::parse_quote!(#[doc = #doc]));
        }
    }

    attrs.extend(doc_comments.iter().cloned());
    attrs
}

fn well_known_default(ty: &syn::Type) -> Option<syn::Expr> {
    let path = match ty {
        syn::Type::Path(syn::TypePath { path, qself: None }) => path,
        _ => return None,
    };

    use syn::parse_quote as pq;

    let ident = path.get_ident()?.to_string();

    let value = match ident.as_str() {
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize" | "i8" | "i16" | "i32" | "i64" | "i128"
        | "isize" => pq!(0),
        "f32" | "f64" => pq!(0.0),
        "bool" => pq!(false),
        "char" => pq!('\0'),
        "String" => pq!(""),
        _ => return None,
    };

    Some(value)
}

/// Unfortunately there is no `syn::Parse` impl for `PatIdent` directly,
/// so we use this workaround instead.
fn pat_ident(ident_name: &'static str) -> syn::PatIdent {
    let ident = syn::Ident::new(ident_name, Span::call_site());
    let pat: syn::Pat = syn::parse_quote!(#ident);
    match pat {
        syn::Pat::Ident(pat_ident) => pat_ident,
        _ => unreachable!("can't parse something else than PatIdent here: {pat:?}"),
    }
}
