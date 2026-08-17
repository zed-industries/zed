use crate::utils::die;
use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    Error, Result,
};

mod kw {
    syn::custom_keyword!(constructor);
    syn::custom_keyword!(error_type);
    syn::custom_keyword!(name);
}

// Example: error_type(name = Foo, constructor = Foo::new)
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Attributes {
    pub(crate) error_type: Option<ErrorTypeAttribute>,
}

// Example: error_type(name = Foo, constructor = Foo::new)
#[cfg_attr(test, derive(Debug))]
pub(crate) enum AttributeItem {
    ErrorType(ErrorTypeAttribute),
}

impl Parse for Attributes {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attribute_items = input.parse_terminated(AttributeItem::parse, syn::Token![,])?;
        let mut maybe_error_type = None;
        for attribute_item in &attribute_items {
            match attribute_item {
                AttributeItem::ErrorType(error_type) => {
                    if maybe_error_type.is_some() {
                        return Err(Error::new(
                            error_type.span,
                            "num_enum attribute must have at most one error_type",
                        ));
                    }
                    maybe_error_type = Some(error_type.clone());
                }
            }
        }
        Ok(Self {
            error_type: maybe_error_type,
        })
    }
}

impl Parse for AttributeItem {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::error_type) {
            input.parse().map(Self::ErrorType)
        } else {
            Err(lookahead.error())
        }
    }
}

// Example: error_type(name = Foo, constructor = Foo::new)
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct ErrorTypeAttribute {
    pub(crate) name: ErrorTypeNameAttribute,
    pub(crate) constructor: ErrorTypeConstructorAttribute,

    span: Span,
}

impl Parse for ErrorTypeAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: kw::error_type = input.parse()?;
        let span = keyword.span;
        let content;
        syn::parenthesized!(content in input);
        let attribute_values =
            content.parse_terminated(ErrorTypeAttributeNamedArgument::parse, syn::Token![,])?;
        let mut name = None;
        let mut constructor = None;
        for attribute_value in &attribute_values {
            match attribute_value {
                ErrorTypeAttributeNamedArgument::Name(name_attr) => {
                    if name.is_some() {
                        die!("num_enum error_type attribute must have exactly one `name` value");
                    }
                    name = Some(name_attr.clone());
                }
                ErrorTypeAttributeNamedArgument::Constructor(constructor_attr) => {
                    if constructor.is_some() {
                        die!("num_enum error_type attribute must have exactly one `constructor` value")
                    }
                    constructor = Some(constructor_attr.clone());
                }
            }
        }
        match (name, constructor) {
            (None, None) => Err(Error::new(
                span,
                "num_enum error_type attribute requires `name` and `constructor` values",
            )),
            (Some(_), None) => Err(Error::new(
                span,
                "num_enum error_type attribute requires `constructor` value",
            )),
            (None, Some(_)) => Err(Error::new(
                span,
                "num_enum error_type attribute requires `name` value",
            )),
            (Some(name), Some(constructor)) => Ok(Self {
                name,
                constructor,
                span,
            }),
        }
    }
}

// Examples:
//  * name = Foo
//  * constructor = Foo::new
pub(crate) enum ErrorTypeAttributeNamedArgument {
    Name(ErrorTypeNameAttribute),
    Constructor(ErrorTypeConstructorAttribute),
}

impl Parse for ErrorTypeAttributeNamedArgument {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::name) {
            input.parse().map(Self::Name)
        } else if lookahead.peek(kw::constructor) {
            input.parse().map(Self::Constructor)
        } else {
            Err(lookahead.error())
        }
    }
}

// Example: name = Foo
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct ErrorTypeNameAttribute {
    pub(crate) path: syn::Path,
}

impl Parse for ErrorTypeNameAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::name>()?;
        input.parse::<syn::Token![=]>()?;
        let path = input.parse()?;
        Ok(Self { path })
    }
}

// Example: constructor = Foo::new
#[derive(Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct ErrorTypeConstructorAttribute {
    pub(crate) path: syn::Path,
}

impl Parse for ErrorTypeConstructorAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<kw::constructor>()?;
        input.parse::<syn::Token![=]>()?;
        let path = input.parse()?;
        Ok(Self { path })
    }
}

#[cfg(test)]
mod test {
    use crate::enum_attributes::Attributes;
    use quote::ToTokens;
    use syn::{parse_quote, Path};

    #[test]
    fn parse_num_enum_attr() {
        let expected_name: Path = parse_quote! { Foo };
        let expected_constructor: Path = parse_quote! { ::foo::Foo::<u8>::new };

        let attributes: Attributes =
            syn::parse_str("error_type(name = Foo, constructor = ::foo::Foo::<u8>::new)").unwrap();
        assert!(attributes.error_type.is_some());
        let error_type = attributes.error_type.unwrap();
        assert_eq!(
            error_type.name.path.to_token_stream().to_string(),
            expected_name.to_token_stream().to_string()
        );
        assert_eq!(
            error_type.constructor.path.to_token_stream().to_string(),
            expected_constructor.to_token_stream().to_string()
        );
    }

    #[test]
    fn parse_num_enum_attr_swapped_order() {
        let expected_name: Path = parse_quote! { Foo };
        let expected_constructor: Path = parse_quote! { ::foo::Foo::<u8>::new };

        let attributes: Attributes =
            syn::parse_str("error_type(constructor = ::foo::Foo::<u8>::new, name = Foo)").unwrap();
        assert!(attributes.error_type.is_some());
        let error_type = attributes.error_type.unwrap();
        assert_eq!(
            error_type.name.path.to_token_stream().to_string(),
            expected_name.to_token_stream().to_string()
        );
        assert_eq!(
            error_type.constructor.path.to_token_stream().to_string(),
            expected_constructor.to_token_stream().to_string()
        );
    }

    #[test]
    fn missing_constructor() {
        let err = syn::parse_str::<Attributes>("error_type(name = Foo)").unwrap_err();
        assert_eq!(
            err.to_string(),
            "num_enum error_type attribute requires `constructor` value"
        );
    }

    #[test]
    fn missing_name() {
        let err = syn::parse_str::<Attributes>("error_type(constructor = Foo::new)").unwrap_err();
        assert_eq!(
            err.to_string(),
            "num_enum error_type attribute requires `name` value"
        );
    }

    #[test]
    fn missing_both() {
        let err = syn::parse_str::<Attributes>("error_type()").unwrap_err();
        assert_eq!(
            err.to_string(),
            "num_enum error_type attribute requires `name` and `constructor` values"
        );
    }

    #[test]
    fn extra_attr() {
        let err = syn::parse_str::<Attributes>(
            "error_type(name = Foo, constructor = Foo::new, extra = unneeded)",
        )
        .unwrap_err();
        assert_eq!(err.to_string(), "expected `name` or `constructor`");
    }

    #[test]
    fn multiple_names() {
        let err = syn::parse_str::<Attributes>(
            "error_type(name = Foo, name = Foo, constructor = Foo::new)",
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "num_enum error_type attribute must have exactly one `name` value"
        );
    }

    #[test]
    fn multiple_constructors() {
        let err = syn::parse_str::<Attributes>(
            "error_type(name = Foo, constructor = Foo::new, constructor = Foo::new)",
        )
        .unwrap_err();
        assert_eq!(
            err.to_string(),
            "num_enum error_type attribute must have exactly one `constructor` value"
        );
    }
}
