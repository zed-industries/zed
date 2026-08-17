// We must make sure this code never panics. It must be able to handle all possible inputs.
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable,
    clippy::unimplemented,
    clippy::todo,
    clippy::exit
)]

use crate::util::prelude::*;
use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream, Parser};
use syn::{token, Token};

pub(crate) fn parse_comma_separated_meta(input: ParseStream<'_>) -> syn::Result<Vec<Meta>> {
    let mut output = vec![];

    while !input.is_empty() {
        let value = input.parse::<Meta>()?;

        output.push(value);

        while !input.is_empty() {
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
                break;
            }

            // Skip the invalid token (comma is expected only).
            input.parse::<TokenTree>()?;
        }
    }

    Ok(output)
}

#[derive(Clone, Debug)]
pub(crate) enum Meta {
    None,
    Path(syn::Path),
    List(MetaList),
    NameValue(MetaNameValue),
}

impl Parse for Meta {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = loop {
            let path = input.call(syn::Path::parse_mod_style).ok();

            if let Some(path) = path {
                break path;
            }

            if input.parse::<TokenTree>().is_err() {
                return Ok(Self::None);
            }
        };

        let meta = if input.peek(token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(token::Bracket) {
            let content;
            syn::bracketed!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);

            Self::List(MetaList {
                path,
                tokens: content.parse()?,
            })
        } else if input.peek(Token![=]) {
            Self::NameValue(MetaNameValue {
                path,
                eq_token: input.parse()?,
                value: input.parse().ok(),
            })
        } else {
            Self::Path(path)
        };

        Ok(meta)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MetaList {
    pub(crate) path: syn::Path,
    pub(crate) tokens: TokenStream,
}

#[derive(Clone, Debug)]
pub(crate) struct MetaNameValue {
    pub(crate) path: syn::Path,

    #[allow(dead_code)]
    pub(crate) eq_token: syn::Token![=],

    #[allow(dead_code)]
    pub(crate) value: Option<syn::Expr>,
}

fn paths_from_meta(meta: Vec<Meta>) -> Vec<syn::Path> {
    meta.into_iter()
        .filter_map(|meta| match meta {
            Meta::Path(path) => Some(path),
            Meta::NameValue(meta) => Some(meta.path),
            Meta::List(meta) => Some(meta.path),
            Meta::None => None,
        })
        .collect()
}

/// This is a highly experimental function that attempts to generate code that
/// can be used by IDEs to provide hints and completions for attributes in macros.
///
/// The idea is that we parse a potentially incomplete syntax of the macro invocation
/// and then we generate a set of modules that contain `use` statements with the
/// identifiers passed to the macro.
///
/// By placing these input identifiers in the right places inside of `use` statements
/// we can hint the IDEs to provide completions for the attributes based on what's
/// available in the module the use statement references.
pub(crate) fn generate_completion_triggers(meta: Vec<Meta>) -> TokenStream {
    let bon = meta
        .iter()
        .find_map(|meta| match meta {
            Meta::NameValue(meta) if meta.path.is_ident("crate") => Some(meta.value.as_ref()),
            _ => None,
        })
        .flatten()
        .and_then(|expr| Some(expr.require_path_mod_style().ok()?.clone()))
        .unwrap_or_else(|| syn::parse_quote!(::bon));

    let completions = CompletionsSchema::with_children(
        "builder_top_level",
        vec![
            CompletionsSchema::leaf("builder_type"),
            CompletionsSchema::leaf("start_fn"),
            CompletionsSchema::leaf("finish_fn"),
            CompletionsSchema::leaf("state_mod"),
            CompletionsSchema::leaf("on").set_custom_filter(|meta| {
                if let Some(first) = meta.first() {
                    if let Meta::Path(path) = first {
                        if path.is_ident("into")
                            || path.is_ident("required")
                            || path.is_ident("overwritable")
                        {
                            return;
                        }
                    }

                    meta.remove(0);
                }
            }),
            CompletionsSchema::leaf("derive"),
        ],
    );

    let completion_triggers = completions.generate_completion_triggers(&bon, meta, &[]);

    quote! {
        // The special `rust_analyzer` CFG is enabled only when Rust Analyzer is
        // running its code analysis. This allows us to provide code that is
        // useful only for Rust Analyzer for it to provide hints and completions.
        #[allow(unexpected_cfgs)]
        const _: () = {
            #[cfg(rust_analyzer)]
            {
                #completion_triggers
            }
        };
    }
}

struct CompletionsSchema {
    key: &'static str,
    children: Vec<Self>,
    custom_filter: Option<fn(&mut Vec<Meta>)>,
}

impl CompletionsSchema {
    fn leaf(key: &'static str) -> Self {
        Self {
            key,
            children: vec![],
            custom_filter: None,
        }
    }

    fn with_children(key: &'static str, children: Vec<Self>) -> Self {
        Self {
            key,
            children,
            custom_filter: None,
        }
    }

    fn set_custom_filter(mut self, custom_filter: fn(&mut Vec<Meta>)) -> Self {
        self.custom_filter = Some(custom_filter);
        self
    }

    fn generate_completion_triggers(
        &self,
        bon: &syn::Path,
        mut meta: Vec<Meta>,
        module_prefix: &[&syn::Ident],
    ) -> TokenStream {
        if let Some(custom_filter) = self.custom_filter {
            custom_filter(&mut meta);
        }

        let module_suffix = syn::Ident::new(self.key, Span::call_site());
        let module_name = module_prefix
            .iter()
            .copied()
            .chain([&module_suffix])
            .collect::<Vec<_>>();

        let child_completion_triggers = self
            .children
            .iter()
            .map(|child| {
                let child_metas = meta
                    .iter()
                    .filter_map(|meta| {
                        let meta = match meta {
                            Meta::List(meta) => meta,
                            _ => return None,
                        };

                        if !meta.path.is_ident(&child.key) {
                            return None;
                        }

                        parse_comma_separated_meta.parse2(meta.tokens.clone()).ok()
                    })
                    .concat();

                child.generate_completion_triggers(bon, child_metas, &module_name)
            })
            .collect::<Vec<_>>();

        let paths = paths_from_meta(meta);
        let module_name_snake_case =
            syn::Ident::new(&module_name.iter().join("_"), Span::call_site());

        quote! {
            mod #module_name_snake_case {
                // We separately import everything from the IDE hints module
                // and then put the `paths` in the `use self::{ ... }` statement
                // to avoid Rust Analyzer from providing completions for the
                // `self` keyword in the `use` statement. It works because
                // `use self::self` is not a valid syntax.
                use #bon::__::ide #(::#module_name)* ::*;
                use self::{ #( #paths as _, )* };
            }

            #(#child_completion_triggers)*
        }
    }
}
