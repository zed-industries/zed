use beef::lean::Cow;
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, GenericParam, Lit, Meta, Type};

use crate::error::Errors;
use crate::leaf::{Callback, InlineCallback};
use crate::util::{expect_punct, MaybeVoid};
use crate::LOGOS_ATTR;

mod definition;
mod ignore_flags;
mod nested;
mod subpattern;
mod type_params;

pub use self::definition::{Definition, Literal};
pub use self::ignore_flags::IgnoreFlags;
use self::nested::{AttributeParser, Nested, NestedValue};
pub use self::subpattern::Subpatterns;
use self::type_params::{replace_lifetime, traverse_type, TypeParams};

#[derive(Default)]
pub struct Parser {
    pub errors: Errors,
    pub mode: Mode,
    pub source: Option<TokenStream>,
    pub skips: Vec<Literal>,
    pub extras: MaybeVoid,
    pub error_type: MaybeVoid,
    pub subpatterns: Subpatterns,
    pub logos_path: Option<TokenStream>,
    types: TypeParams,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Utf8,
    Binary,
}

impl Parser {
    pub fn parse_generic(&mut self, param: GenericParam) {
        match param {
            GenericParam::Lifetime(lt) => {
                self.types.explicit_lifetime(lt, &mut self.errors);
            }
            GenericParam::Type(ty) => {
                self.types.add(ty.ident);
            }
            GenericParam::Const(c) => {
                self.err("Logos doesn't support const generics.", c.span());
            }
        }
    }

    pub fn generics(&mut self) -> Option<TokenStream> {
        self.types.generics(&mut self.errors)
    }

    fn parse_attr(&mut self, attr: &mut Attribute) -> Option<AttributeParser> {
        match &mut attr.meta {
            Meta::List(list) => {
                let tokens = std::mem::replace(&mut list.tokens, TokenStream::new());

                Some(AttributeParser::new(tokens))
            }
            _ => None,
        }
    }

    /// Try to parse the main `#[logos(...)]`, does nothing if
    /// the attribute's name isn't `logos`.
    pub fn try_parse_logos(&mut self, attr: &mut Attribute) {
        if !attr.path().is_ident(LOGOS_ATTR) {
            return;
        }

        let nested = match self.parse_attr(attr) {
            Some(tokens) => tokens,
            None => {
                self.err("Expected #[logos(...)]", attr.span());
                return;
            }
        };

        for nested in nested {
            let (name, value) = match nested {
                Nested::Named(name, value) => (name, value),
                Nested::Unexpected(tokens) | Nested::Unnamed(tokens) => {
                    self.err("Invalid nested attribute", tokens.span());
                    continue;
                }
            };

            // IMPORTANT: Keep these sorted alphabetically for binary search down the line
            #[allow(clippy::type_complexity)]
            static NESTED_LOOKUP: &[(&str, fn(&mut Parser, Span, NestedValue))] = &[
                ("crate", |parser, span, value| match value {
                    NestedValue::Assign(logos_path) => parser.logos_path = Some(logos_path),
                    _ => {
                        parser.err("Expected: #[logos(crate = path::to::logos)]", span);
                    }
                }),
                ("error", |parser, span, value| match value {
                    NestedValue::Assign(value) => {
                        let span = value.span();

                        if let MaybeVoid::Some(previous) = parser.error_type.replace(value) {
                            parser
                                .err("Error type can be defined only once", span)
                                .err("Previous definition here", previous.span());
                        }
                    }
                    _ => {
                        parser.err("Expected: #[logos(error = SomeType)]", span);
                    }
                }),
                ("extras", |parser, span, value| match value {
                    NestedValue::Assign(value) => {
                        let span = value.span();

                        if let MaybeVoid::Some(previous) = parser.extras.replace(value) {
                            parser
                                .err("Extras can be defined only once", span)
                                .err("Previous definition here", previous.span());
                        }
                    }
                    _ => {
                        parser.err("Expected: #[logos(extras = SomeType)]", span);
                    }
                }),
                ("skip", |parser, span, value| match value {
                    NestedValue::Literal(lit) => {
                        if let Some(literal) = parser.parse_literal(Lit::new(lit)) {
                            parser.skips.push(literal);
                        }
                    }
                    _ => {
                        parser.err("Expected: #[logos(skip \"regex literal\")]", span);
                    }
                }),
                ("source", |parser, span, value| match value {
                    NestedValue::Assign(value) => {
                        let span = value.span();
                        if let Some(previous) = parser.source.replace(value) {
                            parser
                                .err("Source can be defined only once", span)
                                .err("Previous definition here", previous.span());
                        }
                    }
                    _ => {
                        parser.err("Expected: #[logos(source = SomeType)]", span);
                    }
                }),
                ("subpattern", |parser, span, value| match value {
                    NestedValue::KeywordAssign(name, value) => {
                        parser.subpatterns.add(name, value, &mut parser.errors);
                    }
                    _ => {
                        parser.err(r#"Expected: #[logos(subpattern name = r"regex")]"#, span);
                    }
                }),
                ("type", |parser, span, value| match value {
                    NestedValue::KeywordAssign(generic, ty) => {
                        parser.types.set(generic, ty, &mut parser.errors);
                    }
                    _ => {
                        parser.err("Expected: #[logos(type T = SomeType)]", span);
                    }
                }),
            ];

            match NESTED_LOOKUP.binary_search_by_key(&name.to_string().as_str(), |(n, _)| n) {
                Ok(idx) => NESTED_LOOKUP[idx].1(self, name.span(), value),
                Err(_) => {
                    let mut err = format!(
                        "Unknown nested attribute #[logos({name})], expected one of: {}",
                        NESTED_LOOKUP[0].0
                    );

                    for (allowed, _) in &NESTED_LOOKUP[1..] {
                        err.push_str(", ");
                        err.push_str(allowed);
                    }

                    self.err(err, name.span());
                }
            }
        }
    }

    pub fn parse_literal(&mut self, lit: Lit) -> Option<Literal> {
        match lit {
            Lit::Str(string) => Some(Literal::Utf8(string)),
            Lit::ByteStr(bytes) => {
                self.mode = Mode::Binary;

                Some(Literal::Bytes(bytes))
            }
            _ => {
                self.err("Expected a &str or &[u8] slice", lit.span());

                None
            }
        }
    }

    /// Parse attribute definition of a token:
    ///
    /// + `#[token(literal[, callback])]`
    /// + `#[regex(literal[, callback])]`
    pub fn parse_definition(&mut self, attr: &mut Attribute) -> Option<Definition> {
        let mut nested = self.parse_attr(attr)?;

        let literal = match nested.parsed::<Lit>()? {
            Ok(lit) => self.parse_literal(lit)?,
            Err(err) => {
                self.err(err.to_string(), err.span());

                return None;
            }
        };

        let mut def = Definition::new(literal);

        for (position, next) in nested.enumerate() {
            match next {
                Nested::Unexpected(tokens) => {
                    self.err("Unexpected token in attribute", tokens.span());
                }
                Nested::Unnamed(tokens) => match position {
                    0 => def.callback = self.parse_callback(tokens),
                    _ => {
                        self.err(
                            "\
                            Expected a named argument at this position\n\
                            \n\
                            hint: If you are trying to define a callback here use: callback = ...\
                            ",
                            tokens.span(),
                        );
                    }
                },
                Nested::Named(name, value) => {
                    def.named_attr(name, value, self);
                }
            }
        }

        Some(def)
    }

    fn parse_callback(&mut self, tokens: TokenStream) -> Option<Callback> {
        let span = tokens.span();
        let mut tokens = tokens.into_iter();

        if let Some(tt) = expect_punct(tokens.next(), '|') {
            let mut label = TokenStream::from(tt);

            label.extend(tokens);

            return Some(Callback::Label(label));
        }

        let first = tokens.next();
        let error = expect_punct(tokens.next(), '|');

        let arg = match (error, first) {
            (None, Some(TokenTree::Ident(arg))) => arg,
            _ => {
                self.err(
                    "Inline callbacks must use closure syntax with exactly one parameter",
                    span,
                );
                return None;
            }
        };

        let body = match tokens.next() {
            Some(TokenTree::Group(group)) => group.stream(),
            Some(first) => {
                let mut body = TokenStream::from(first);

                body.extend(tokens);
                body
            }
            None => {
                self.err("Callback missing a body", span);
                return None;
            }
        };

        let inline = InlineCallback { arg, body, span };

        Some(inline.into())
    }

    /// Checks if `ty` is a declared generic param, if so replaces it
    /// with a concrete type defined using #[logos(type T = Type)]
    ///
    /// If no matching generic param is found, all lifetimes are fixed
    /// to the source lifetime
    pub fn get_type(&self, ty: &mut Type) -> TokenStream {
        traverse_type(ty, &mut |ty| {
            if let Type::Path(tp) = ty {
                // Skip types that begin with `self::`
                if tp.qself.is_none() {
                    // If `ty` is a generic type parameter, try to find
                    // its concrete type defined with #[logos(type T = Type)]
                    if let Some(substitute) = self.types.find(&tp.path) {
                        *ty = substitute;
                    }
                }
            }
            // If `ty` is a concrete type, fix its lifetimes to 'source
            replace_lifetime(ty);
        });

        quote!(#ty)
    }

    pub fn err<M>(&mut self, message: M, span: Span) -> &mut Errors
    where
        M: Into<Cow<'static, str>>,
    {
        self.errors.err(message, span)
    }
}
