use proc_macro2::Span;
use syn::{parse_quote, spanned::Spanned};

use crate::{Error, FromMeta, Result};

mod core;
mod forward_attrs;
mod from_attributes;
mod from_derive;
mod from_field;
mod from_meta;
mod from_type_param;
mod from_variant;
mod input_field;
mod input_variant;
mod outer_from;
mod shape;

pub use self::core::Core;
pub use self::forward_attrs::ForwardAttrs;
pub use self::from_attributes::FromAttributesOptions;
pub use self::from_derive::FdiOptions;
pub use self::from_field::FromFieldOptions;
pub use self::from_meta::FromMetaOptions;
pub use self::from_type_param::FromTypeParamOptions;
pub use self::from_variant::FromVariantOptions;
pub use self::input_field::InputField;
pub use self::input_variant::InputVariant;
pub use self::outer_from::OuterFrom;
pub use self::shape::{DataShape, DeriveInputShapeSet};

/// A default/fallback expression encountered in attributes during parsing.
#[derive(Debug, Clone)]
pub enum DefaultExpression {
    /// The value should be taken from the `default` instance of the containing struct.
    /// This is not valid in container options.
    Inherit,
    Explicit(syn::Path),
    Trait {
        /// The input span that is responsible for the use of `Default::default`.
        span: Span,
    },
}

#[doc(hidden)]
impl FromMeta for DefaultExpression {
    // Note: This cannot use `from_word` as it needs to capture the span
    // in the `Meta::Path` case.
    fn from_meta(item: &syn::Meta) -> Result<Self> {
        match item {
            syn::Meta::Path(_) => Ok(DefaultExpression::Trait { span: item.span() }),
            syn::Meta::List(nm) => Err(Error::unsupported_format("list").with_span(nm)),
            syn::Meta::NameValue(nv) => Self::from_value(&nv.lit),
        }
    }

    fn from_value(value: &syn::Lit) -> Result<Self> {
        syn::Path::from_value(value).map(DefaultExpression::Explicit)
    }
}

/// Middleware for extracting attribute values. Implementers are expected to override
/// `parse_nested` so they can apply individual items to themselves, while `parse_attributes`
/// is responsible for looping through distinct outer attributes and collecting errors.
pub trait ParseAttribute: Sized {
    fn parse_attributes(mut self, attrs: &[syn::Attribute]) -> Result<Self> {
        let mut errors = Error::accumulator();
        for attr in attrs {
            if attr.path == parse_quote!(darling) {
                errors.handle(parse_attr(attr, &mut self));
            }
        }

        errors.finish_with(self)
    }

    /// Read a meta-item, and apply its values to the current instance.
    fn parse_nested(&mut self, mi: &syn::Meta) -> Result<()>;
}

fn parse_attr<T: ParseAttribute>(attr: &syn::Attribute, target: &mut T) -> Result<()> {
    let mut errors = Error::accumulator();
    match attr.parse_meta().ok() {
        Some(syn::Meta::List(data)) => {
            for item in data.nested {
                if let syn::NestedMeta::Meta(ref mi) = item {
                    errors.handle(target.parse_nested(mi));
                } else {
                    panic!("Wasn't able to parse: `{:?}`", item);
                }
            }

            errors.finish()
        }
        Some(ref item) => panic!("Wasn't able to parse: `{:?}`", item),
        None => panic!("Unable to parse {:?}", attr),
    }
}

/// Middleware for extracting values from the body of the derive input. Implementers are
/// expected to override `parse_field` or `parse_variant` as appropriate for their use-case,
/// while `parse_body` dispatches to the appropriate methods and handles error collection.
pub trait ParseData: Sized {
    fn parse_body(mut self, body: &syn::Data) -> Result<Self> {
        use syn::{Data, Fields};

        let mut errors = Error::accumulator();

        match *body {
            Data::Struct(ref data) => match data.fields {
                Fields::Unit => {}
                Fields::Named(ref fields) => {
                    for field in &fields.named {
                        errors.handle(self.parse_field(field));
                    }
                }
                Fields::Unnamed(ref fields) => {
                    for field in &fields.unnamed {
                        errors.handle(self.parse_field(field));
                    }
                }
            },
            Data::Enum(ref data) => {
                for variant in &data.variants {
                    errors.handle(self.parse_variant(variant));
                }
            }
            Data::Union(_) => unreachable!(),
        };

        errors.finish_with(self)
    }

    /// Apply the next found variant to the object, returning an error
    /// if parsing goes wrong.
    fn parse_variant(&mut self, variant: &syn::Variant) -> Result<()> {
        Err(Error::unsupported_format("enum variant").with_span(variant))
    }

    /// Apply the next found struct field to the object, returning an error
    /// if parsing goes wrong.
    fn parse_field(&mut self, field: &syn::Field) -> Result<()> {
        Err(Error::unsupported_format("struct field").with_span(field))
    }
}
