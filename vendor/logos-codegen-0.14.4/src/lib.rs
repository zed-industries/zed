//! <img src="https://raw.githubusercontent.com/maciejhirsz/logos/master/logos.svg?sanitize=true" alt="Logos logo" width="250" align="right">
//!
//! # Logos
//!
//! This is a `#[derive]` macro crate, [for documentation go to main crate](https://docs.rs/logos).

// The `quote!` macro requires deep recursion.
#![recursion_limit = "196"]
#![doc(html_logo_url = "https://maciej.codes/kosz/logos.png")]

mod error;
mod generator;
#[cfg(not(feature = "fuzzing"))]
mod graph;
#[cfg(feature = "fuzzing")]
pub mod graph;
mod leaf;
#[cfg(not(feature = "fuzzing"))]
mod mir;
#[cfg(feature = "fuzzing")]
pub mod mir;
mod parser;
mod util;

#[macro_use]
#[allow(missing_docs)]
mod macros;

use generator::Generator;
use graph::{DisambiguationError, Fork, Graph, Rope};
use leaf::Leaf;
use parser::{IgnoreFlags, Mode, Parser};
use quote::ToTokens;
use util::MaybeVoid;

use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::parse_quote;
use syn::spanned::Spanned;
use syn::{Fields, ItemEnum};

const LOGOS_ATTR: &str = "logos";
const ERROR_ATTR: &str = "error";
const TOKEN_ATTR: &str = "token";
const REGEX_ATTR: &str = "regex";

/// Generate a `Logos` implementation for the given struct, provided as a stream of rust tokens.
pub fn generate(input: TokenStream) -> TokenStream {
    debug!("Reading input token streams");

    let mut item: ItemEnum = syn::parse2(input).expect("Logos can be only be derived for enums");

    let name = &item.ident;

    let mut parser = Parser::default();

    for param in item.generics.params {
        parser.parse_generic(param);
    }

    for attr in &mut item.attrs {
        parser.try_parse_logos(attr);
    }

    let mut ropes = Vec::new();
    let mut regex_ids = Vec::new();
    let mut graph = Graph::new();

    {
        let errors = &mut parser.errors;

        for literal in &parser.skips {
            match literal.to_mir(&parser.subpatterns, IgnoreFlags::Empty, errors) {
                Ok(mir) => {
                    let then = graph.push(Leaf::new_skip(literal.span()).priority(mir.priority()));
                    let id = graph.regex(mir, then);

                    regex_ids.push(id);
                }
                Err(err) => {
                    errors.err(err, literal.span());
                }
            }
        }
    }

    debug!("Iterating through enum variants");

    for variant in &mut item.variants {
        let field = match &mut variant.fields {
            Fields::Unit => MaybeVoid::Void,
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    parser.err(
                        format!(
                            "Logos currently only supports variants with one field, found {}",
                            fields.unnamed.len(),
                        ),
                        fields.span(),
                    );
                }

                let ty = &mut fields
                    .unnamed
                    .first_mut()
                    .expect("Already checked len; qed")
                    .ty;
                let ty = parser.get_type(ty);

                MaybeVoid::Some(ty)
            }
            Fields::Named(fields) => {
                parser.err("Logos doesn't support named fields yet.", fields.span());

                MaybeVoid::Void
            }
        };

        // Lazy leaf constructor to avoid cloning
        let var_ident = &variant.ident;
        let leaf = move |span| Leaf::new(var_ident, span).field(field.clone());

        for attr in &mut variant.attrs {
            let attr_name = match attr.path().get_ident() {
                Some(ident) => ident.to_string(),
                None => continue,
            };

            match attr_name.as_str() {
                ERROR_ATTR => {
                    // TODO: Remove in future versions
                    parser.err(
                        "\
                        Since 0.13 Logos no longer requires the #[error] variant.\n\
                        \n\
                        For help with migration see release notes: \
                        https://github.com/maciejhirsz/logos/releases\
                        ",
                        attr.span(),
                    );
                }
                TOKEN_ATTR => {
                    let definition = match parser.parse_definition(attr) {
                        Some(definition) => definition,
                        None => {
                            parser.err("Expected #[token(...)]", attr.span());
                            continue;
                        }
                    };

                    if definition.ignore_flags.is_empty() {
                        let bytes = definition.literal.to_bytes();
                        let then = graph.push(
                            leaf(definition.literal.span())
                                .priority(definition.priority.unwrap_or(bytes.len() * 2))
                                .callback(definition.callback),
                        );

                        ropes.push(Rope::new(bytes, then));
                    } else {
                        let mir = definition
                            .literal
                            .escape_regex()
                            .to_mir(
                                &Default::default(),
                                definition.ignore_flags,
                                &mut parser.errors,
                            )
                            .expect("The literal should be perfectly valid regex");

                        let then = graph.push(
                            leaf(definition.literal.span())
                                .priority(definition.priority.unwrap_or_else(|| mir.priority()))
                                .callback(definition.callback),
                        );
                        let id = graph.regex(mir, then);

                        regex_ids.push(id);
                    }
                }
                REGEX_ATTR => {
                    let definition = match parser.parse_definition(attr) {
                        Some(definition) => definition,
                        None => {
                            parser.err("Expected #[regex(...)]", attr.span());
                            continue;
                        }
                    };
                    let mir = match definition.literal.to_mir(
                        &parser.subpatterns,
                        definition.ignore_flags,
                        &mut parser.errors,
                    ) {
                        Ok(mir) => mir,
                        Err(err) => {
                            parser.err(err, definition.literal.span());
                            continue;
                        }
                    };

                    let then = graph.push(
                        leaf(definition.literal.span())
                            .priority(definition.priority.unwrap_or_else(|| mir.priority()))
                            .callback(definition.callback),
                    );
                    let id = graph.regex(mir, then);

                    regex_ids.push(id);
                }
                _ => (),
            }
        }
    }

    let mut root = Fork::new();

    debug!("Parsing additional options (extras, source, ...)");

    let error_type = parser.error_type.take();
    let extras = parser.extras.take();
    let source = parser
        .source
        .take()
        .map(strip_wrapping_parens)
        .unwrap_or(match parser.mode {
            Mode::Utf8 => quote!(str),
            Mode::Binary => quote!([u8]),
        });
    let logos_path = parser
        .logos_path
        .take()
        .unwrap_or_else(|| parse_quote!(::logos));

    let generics = parser.generics();
    let this = quote!(#name #generics);

    let impl_logos = |body| {
        quote! {
            impl<'s> #logos_path::Logos<'s> for #this {
                type Error = #error_type;

                type Extras = #extras;

                type Source = #source;

                fn lex(lex: &mut #logos_path::Lexer<'s, Self>) {
                    #body
                }
            }
        }
    };

    for id in regex_ids {
        let fork = graph.fork_off(id);

        root.merge(fork, &mut graph);
    }
    for rope in ropes {
        root.merge(rope.into_fork(&mut graph), &mut graph);
    }
    while let Some(id) = root.miss.take() {
        let fork = graph.fork_off(id);

        if fork.branches().next().is_some() {
            root.merge(fork, &mut graph);
        } else {
            break;
        }
    }

    debug!("Checking if any two tokens have the same priority");

    for &DisambiguationError(a, b) in graph.errors() {
        let a = graph[a].unwrap_leaf();
        let b = graph[b].unwrap_leaf();
        let disambiguate = a.priority + 1;

        let mut err = |a: &Leaf, b: &Leaf| {
            parser.err(
                format!(
                    "\
                    A definition of variant `{a}` can match the same input as another definition of variant `{b}`.\n\
                    \n\
                    hint: Consider giving one definition a higher priority: \
                    #[regex(..., priority = {disambiguate})]\
                    ",
                ),
                a.span
            );
        };

        err(a, b);
        err(b, a);
    }

    if let Some(errors) = parser.errors.render() {
        return impl_logos(errors);
    }

    let root = graph.push(root);

    graph.shake(root);

    debug!("Generating code from graph:\n{graph:#?}");

    let generator = Generator::new(name, &this, root, &graph);

    let body = generator.generate();
    impl_logos(quote! {
        use #logos_path::internal::{LexerInternal, CallbackResult};

        type Lexer<'s> = #logos_path::Lexer<'s, #this>;

        fn _end<'s>(lex: &mut Lexer<'s>) {
            lex.end()
        }

        fn _error<'s>(lex: &mut Lexer<'s>) {
            lex.bump_unchecked(1);

            lex.error();
        }

        #body
    })
}

/// Strip all logos attributes from the given struct, allowing it to be used in code without `logos-derive` present.
pub fn strip_attributes(input: TokenStream) -> TokenStream {
    let mut item: ItemEnum = syn::parse2(input).expect("Logos can be only be derived for enums");

    strip_attrs_from_vec(&mut item.attrs);

    for attr in &mut item.attrs {
        if let syn::Meta::List(meta) = &mut attr.meta {
            if meta.path.is_ident("derive") {
                let mut tokens =
                    std::mem::replace(&mut meta.tokens, TokenStream::new()).into_iter();

                while let Some(TokenTree::Ident(ident)) = tokens.next() {
                    let punct = tokens.next();

                    if ident == "Logos" {
                        continue;
                    }

                    meta.tokens.extend([TokenTree::Ident(ident)]);
                    meta.tokens.extend(punct);
                }
            }
        }
    }

    for variant in &mut item.variants {
        strip_attrs_from_vec(&mut variant.attrs);
        for field in &mut variant.fields {
            strip_attrs_from_vec(&mut field.attrs);
        }
    }

    item.to_token_stream()
}

fn strip_attrs_from_vec(attrs: &mut Vec<syn::Attribute>) {
    attrs.retain(|attr| !is_logos_attr(attr))
}

fn is_logos_attr(attr: &syn::Attribute) -> bool {
    attr.path().is_ident(LOGOS_ATTR)
        || attr.path().is_ident(TOKEN_ATTR)
        || attr.path().is_ident(REGEX_ATTR)
}

fn strip_wrapping_parens(t: TokenStream) -> TokenStream {
    let tts: Vec<TokenTree> = t.into_iter().collect();

    if tts.len() != 1 {
        tts.into_iter().collect()
    } else {
        match tts.into_iter().next().unwrap() {
            TokenTree::Group(g) => {
                if g.delimiter() == Delimiter::Parenthesis {
                    g.stream()
                } else {
                    core::iter::once(TokenTree::Group(g)).collect()
                }
            }
            tt => core::iter::once(tt).collect(),
        }
    }
}
