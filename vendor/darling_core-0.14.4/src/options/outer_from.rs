use syn::spanned::Spanned;
use syn::{Field, Ident, Meta};

use crate::options::{Core, DefaultExpression, ForwardAttrs, ParseAttribute, ParseData};
use crate::util::PathList;
use crate::{FromMeta, Result};

/// Reusable base for `FromDeriveInput`, `FromVariant`, `FromField`, and other top-level
/// `From*` traits.
#[derive(Debug, Clone)]
pub struct OuterFrom {
    /// The field on the target struct which should receive the type identifier, if any.
    pub ident: Option<Ident>,

    /// The field on the target struct which should receive the type attributes, if any.
    pub attrs: Option<Ident>,

    pub container: Core,

    /// The attribute names that should be searched.
    pub attr_names: PathList,

    /// The attribute names that should be forwarded. The presence of the word with no additional
    /// filtering will cause _all_ attributes to be cloned and exposed to the struct after parsing.
    pub forward_attrs: Option<ForwardAttrs>,

    /// Whether or not the container can be made through conversion from the type `Ident`.
    pub from_ident: bool,
}

impl OuterFrom {
    pub fn start(di: &syn::DeriveInput) -> Result<Self> {
        Ok(OuterFrom {
            container: Core::start(di)?,
            attrs: Default::default(),
            ident: Default::default(),
            attr_names: Default::default(),
            forward_attrs: Default::default(),
            from_ident: Default::default(),
        })
    }
}

impl ParseAttribute for OuterFrom {
    fn parse_nested(&mut self, mi: &Meta) -> Result<()> {
        let path = mi.path();
        if path.is_ident("attributes") {
            self.attr_names = FromMeta::from_meta(mi)?;
        } else if path.is_ident("forward_attrs") {
            self.forward_attrs = FromMeta::from_meta(mi)?;
        } else if path.is_ident("from_ident") {
            // HACK: Declaring that a default is present will cause fields to
            // generate correct code, but control flow isn't that obvious.
            self.container.default = Some(DefaultExpression::Trait {
                // Use the span of the `from_ident` keyword so that errors in generated code
                // caused by this will point back to the correct location.
                span: path.span(),
            });
            self.from_ident = true;
        } else {
            return self.container.parse_nested(mi);
        }
        Ok(())
    }
}

impl ParseData for OuterFrom {
    fn parse_field(&mut self, field: &Field) -> Result<()> {
        match field
            .ident
            .as_ref()
            .map(|v| v.to_string())
            .as_ref()
            .map(|v| v.as_str())
        {
            Some("ident") => {
                self.ident = field.ident.clone();
                Ok(())
            }
            Some("attrs") => {
                self.attrs = field.ident.clone();
                Ok(())
            }
            _ => self.container.parse_field(field),
        }
    }
}
