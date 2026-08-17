use crate::util::prelude::*;
use darling::util::Flag;
use darling::FromMeta;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Debug)]
pub(crate) struct OnConfig {
    pub(crate) type_pattern: syn::Type,
    pub(crate) into: Flag,
    pub(crate) overwritable: Flag,
    pub(crate) required: Flag,
    pub(crate) setters: OnSettersConfig,
}

#[derive(Debug, Default, FromMeta)]
pub(crate) struct OnSettersConfig {
    #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) doc: OnSettersDocConfig,
}

#[derive(Debug, Default, FromMeta)]
pub(crate) struct OnSettersDocConfig {
    #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) default: OnSettersDocDefaultConfig,
}

#[derive(Debug, Default, FromMeta)]
pub(crate) struct OnSettersDocDefaultConfig {
    pub(crate) skip: Flag,
}

impl Parse for OnConfig {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let type_pattern = input.parse()?;

        let comma = input.parse::<syn::Token![,]>()?;
        let rest: TokenStream = input.parse()?;

        #[derive(FromMeta)]
        struct Parsed {
            into: Flag,
            overwritable: Flag,
            required: Flag,

            #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
            setters: OnSettersConfig,
        }

        if rest.is_empty() {
            return Err(syn::Error::new(
                comma.span(),
                "expected at least one parameter after the comma in `on(type_pattern, ...)`",
            ));
        }

        let parsed: Parsed = crate::parsing::parse_non_empty_paren_meta_list(
            &syn::parse_quote_spanned!(comma.span=> on(#rest)),
        )?;

        if !cfg!(feature = "experimental-overwritable") && parsed.overwritable.is_present() {
            return Err(syn::Error::new(
                parsed.overwritable.span(),
                "🔬 `overwritable` attribute is experimental and requires \
                 \"experimental-overwritable\" cargo feature to be enabled; \
                 we would be glad to make this attribute stable if you find it useful; \
                 please leave a 👍 reaction under the issue https://github.com/elastio/bon/issues/149 \
                 to help us measure the demand for this feature; it would be \
                 double-awesome if you could also describe your use case in \
                 a comment under the issue for us to understand how it's used \
                 in practice",
            ));
        }

        struct FindAttr {
            attr: Option<Span>,
        }

        impl Visit<'_> for FindAttr {
            fn visit_attribute(&mut self, attr: &'_ syn::Attribute) {
                self.attr.get_or_insert_with(|| attr.span());
            }
        }

        let mut find_attr = FindAttr { attr: None };
        find_attr.visit_type(&type_pattern);

        if let Some(attr) = find_attr.attr {
            return Err(syn::Error::new(
                attr,
                "nested attributes are not allowed in the type pattern of \
                #[builder(on(type_pattern, ...))]",
            ));
        }

        // The validation is done in the process of matching the types. To make
        // sure that matching traverses the full pattern we match it with itself.
        let type_pattern_matches_itself = type_pattern.matches(&type_pattern)?;

        assert!(
            type_pattern_matches_itself,
            "BUG: the type pattern does not match itself: {type_pattern:#?}"
        );

        Ok(Self {
            type_pattern,
            into: parsed.into,
            overwritable: parsed.overwritable,
            required: parsed.required,
            setters: parsed.setters,
        })
    }
}

impl FromMeta for OnConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => bail!(
                meta,
                "expected an attribute of form `on(type_pattern, ...)`"
            ),
        };

        let me = syn::parse2(meta.tokens.clone())?;

        Ok(me)
    }
}
