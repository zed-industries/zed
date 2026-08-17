use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream, Parser, Result},
    token::Comma,
};
use syn::{Attribute, Fields, Ident, Index, LitStr, Meta, Token, Type};

pub use self::field_attributes::*;
pub use self::type_item_attributes::*;

mod field_attributes;
mod type_item_attributes;

pub fn parse_namespaced_attributes<T: AttributeArgumentParser>(
    attributes: Vec<Attribute>,
) -> (T, Vec<::syn::parse::Error>) {
    let mut result = T::default();
    let mut errors = Vec::new();

    for attribute in attributes {
        let is_palette_attribute = attribute
            .meta
            .path()
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        let meta_list = match attribute.meta.require_list() {
            Ok(list) => list,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if meta_list.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path().span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result =
            Punctuated::<_, Comma>::parse_terminated.parse2(meta_list.tokens.clone());
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    if let Err(new_error) = result.argument(argument) {
                        errors.extend(new_error);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    (result, errors)
}

pub fn parse_field_attributes<T: FieldAttributeArgumentParser>(
    fields: Fields,
) -> (T, Vec<::syn::parse::Error>) {
    let mut result = T::default();
    let mut errors = Vec::new();

    let attributes = fields.into_iter().enumerate().flat_map(|(index, field)| {
        let field_name = field
            .ident
            .map(IdentOrIndex::Ident)
            .unwrap_or_else(|| IdentOrIndex::Index(index.into()));
        let ty = field.ty;

        field
            .attrs
            .into_iter()
            .map(move |attribute| (field_name.clone(), ty.clone(), attribute))
    });

    for (field_name, ty, attribute) in attributes {
        let is_palette_attribute = attribute
            .path()
            .get_ident()
            .map(|name| name == "palette")
            .unwrap_or(false);

        if !is_palette_attribute {
            continue;
        }

        let meta_list = match attribute.meta.require_list() {
            Ok(list) => list,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };

        if meta_list.tokens.is_empty() {
            errors.push(::syn::parse::Error::new(
                attribute.path().span(),
                "expected `palette(...)`",
            ));

            continue;
        }

        let parse_result =
            Punctuated::<_, Comma>::parse_terminated.parse2(meta_list.tokens.clone());
        match parse_result {
            Ok(meta) => {
                for argument in meta {
                    if let Err(new_errors) = result.argument(&field_name, &ty, argument) {
                        errors.extend(new_errors);
                    }
                }
            }
            Err(error) => errors.push(error),
        }
    }

    (result, errors)
}

pub fn assert_path_meta(meta: &Meta) -> Result<()> {
    if !matches!(meta, Meta::Path(_)) {
        return Err(::syn::parse::Error::new(
            meta.span(),
            "expected the attribute to be just an identifier or a path",
        ));
    }

    Ok(())
}

#[derive(PartialEq)]
pub struct KeyValuePair {
    pub key: Ident,
    pub value: Ident,
}

impl Parse for KeyValuePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse::<LitStr>()?.parse::<Ident>()?;
        Ok(KeyValuePair { key, value })
    }
}

impl PartialEq<str> for KeyValuePair {
    fn eq(&self, other: &str) -> bool {
        self.key == other
    }
}

#[derive(Clone)]
pub enum IdentOrIndex {
    Index(Index),
    Ident(Ident),
}

impl PartialEq for IdentOrIndex {
    fn eq(&self, other: &IdentOrIndex) -> bool {
        match (self, other) {
            (IdentOrIndex::Index(this), IdentOrIndex::Index(other)) => this.index == other.index,
            (IdentOrIndex::Ident(this), IdentOrIndex::Ident(other)) => this == other,
            _ => false,
        }
    }
}

impl Eq for IdentOrIndex {}

impl ::std::hash::Hash for IdentOrIndex {
    fn hash<H: ::std::hash::Hasher>(&self, hasher: &mut H) {
        ::std::mem::discriminant(self).hash(hasher);

        match *self {
            IdentOrIndex::Index(ref index) => index.index.hash(hasher),
            IdentOrIndex::Ident(ref ident) => ident.hash(hasher),
        }
    }
}

impl ::quote::ToTokens for IdentOrIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            IdentOrIndex::Index(ref index) => index.to_tokens(tokens),
            IdentOrIndex::Ident(ref ident) => ident.to_tokens(tokens),
        }
    }
}

pub trait AttributeArgumentParser: Default {
    fn argument(&mut self, argument: Meta) -> std::result::Result<(), Vec<syn::Error>>;
}

pub trait FieldAttributeArgumentParser: Default {
    fn argument(
        &mut self,
        field_name: &IdentOrIndex,
        ty: &Type,
        argument: Meta,
    ) -> std::result::Result<(), Vec<syn::Error>>;
}
