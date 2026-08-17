mod config;
mod into_conversion;
mod named;

pub(crate) use config::*;
pub(crate) use named::*;

use super::top_level_config::OnConfig;
use super::TopLevelConfig;
use crate::normalization::SyntaxVariant;
use crate::util::prelude::*;
use config::MemberConfig;
use darling::FromAttributes;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub(crate) enum MemberOrigin {
    FnArg,
    StructField,
}

impl fmt::Display for MemberOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FnArg => write!(f, "function argument"),
            Self::StructField => write!(f, "struct field"),
        }
    }
}

impl MemberOrigin {
    fn parent_construct(self) -> &'static str {
        match self {
            Self::FnArg => "function",
            Self::StructField => "struct",
        }
    }
}

#[derive(Debug)]
pub(crate) enum Member {
    /// Member that was marked with `#[builder(start_fn)]`
    StartFn(PosFnMember),

    /// Member that was marked with `#[builder(field)]`
    Field(CustomField),

    /// Member that was marked with `#[builder(finish_fn)]`
    FinishFn(PosFnMember),

    /// Regular named member included in the typestate.
    Named(NamedMember),

    /// Member that was marked with `#[builder(skip)]`
    Skip(SkipMember),
}

#[derive(Debug)]
pub(crate) struct CustomField {
    pub(crate) ident: syn::Ident,
    pub(crate) norm_ty: Box<syn::Type>,

    /// Initial value of the field
    pub(crate) init: Option<syn::Expr>,
}

#[derive(Debug)]
pub(crate) struct PosFnMember {
    /// Specifies what syntax the member comes from.
    pub(crate) origin: MemberOrigin,

    /// Original identifier of the member
    pub(crate) ident: syn::Ident,

    /// Type of the member
    pub(crate) ty: SyntaxVariant<Box<syn::Type>>,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) config: MemberConfig,
}

/// Member that was skipped by the user with `#[builder(skip)]`
#[derive(Debug)]
pub(crate) struct SkipMember {
    pub(crate) ident: syn::Ident,

    /// Normalized type of the member
    pub(crate) norm_ty: Box<syn::Type>,

    /// Value to assign to the member
    pub(crate) value: Option<syn::Expr>,
}

pub(crate) struct RawMember<'a> {
    pub(crate) attrs: &'a [syn::Attribute],
    pub(crate) ident: syn::Ident,
    pub(crate) ty: SyntaxVariant<Box<syn::Type>>,
    pub(crate) span: Span,
}

impl Member {
    // False-positive lint. We can't elide the lifetime in `RawMember` because
    // anonymous lifetimes in impl traits are unstable, and we shouldn't omit
    // the lifetime parameter because we want to be explicit about its existence
    // (there is an other lint that checks for this).
    #[allow(single_use_lifetimes)]
    pub(crate) fn from_raw<'a>(
        top_config: &TopLevelConfig,
        origin: MemberOrigin,
        members: impl IntoIterator<Item = RawMember<'a>>,
    ) -> Result<Vec<Self>> {
        let on = &top_config.on;

        let mut members = members
            .into_iter()
            .map(|member| {
                for attr in member.attrs {
                    if attr.meta.path().is_ident("builder") {
                        crate::parsing::require_non_empty_paren_meta_list_or_name_value(
                            &attr.meta,
                        )?;
                    }
                }

                let config = MemberConfig::from_attributes(member.attrs)?;
                config.validate(top_config, origin)?;
                Ok((member, config))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .peekable();

        let mut output = vec![];

        // Collect `start_fn` members
        while let Some((member, config)) = members.next_if(|(_, cfg)| cfg.start_fn.is_present()) {
            let member = PosFnMember::new(origin, member, on, config)?;
            output.push(Self::StartFn(member));
        }

        // Collect `field` members
        while let Some((member, config)) = members.next_if(|(_, cfg)| cfg.field.is_some()) {
            let init = config
                .field
                .expect("validated `field.is_some()` in `next_if`")
                .value;

            let member = CustomField::new(member, init)?;
            output.push(Self::Field(member));
        }

        // Collect `finish_fn` members
        while let Some((member, cfg)) = members.next_if(|(_, cfg)| cfg.finish_fn.is_present()) {
            let member = PosFnMember::new(origin, member, on, cfg)?;
            output.push(Self::FinishFn(member));
        }

        let mut named_count = 0;

        for (member, config) in members {
            let RawMember {
                attrs,
                ident,
                ty,
                span: _,
            } = member;

            if let Some(value) = config.skip {
                output.push(Self::Skip(SkipMember {
                    ident,
                    norm_ty: ty.norm,
                    value: value.value,
                }));
                continue;
            }

            let active_flag = |flag: darling::util::Flag| flag.is_present().then(|| flag.span());

            let incorrect_order = None
                .or_else(|| active_flag(config.start_fn))
                .or_else(|| Some(config.field.as_ref()?.key.span()))
                .or_else(|| active_flag(config.finish_fn));

            if let Some(span) = incorrect_order {
                bail!(
                    &span,
                    "incorrect members ordering; expected ordering:\n\
                    (1) members annotated with #[builder(start_fn)]\n\
                    (2) members annotated with #[builder(field)]\n\
                    (3) members annotated with #[builder(finish_fn)]\n\
                    (4) all other members in any order",
                );
            }

            // XXX: docs are collected only for named members. There is no obvious
            // place where to put the docs for positional and skipped members.
            //
            // Even if there are some docs on them and the function syntax is used
            // then these docs will just be removed from the output function.
            // It's probably fine since the doc comments are there in the code
            // itself which is also useful for people reading the source code.
            let docs = attrs
                .iter()
                .filter(|attr| attr.is_doc_expr())
                .cloned()
                .collect();

            let mut member = NamedMember {
                index: named_count.into(),
                origin,
                name: MemberName::new(ident, &config),
                ty,
                config,
                docs,
                span: member.span,
            };

            member.merge_on_config(on)?;
            member.validate()?;

            output.push(Self::Named(member));
            named_count += 1;
        }

        Ok(output)
    }
}

impl Member {
    pub(crate) fn norm_ty(&self) -> &syn::Type {
        match self {
            Self::Field(me) => &me.norm_ty,
            Self::FinishFn(me) | Self::StartFn(me) => &me.ty.norm,
            Self::Named(me) => &me.ty.norm,
            Self::Skip(me) => &me.norm_ty,
        }
    }

    pub(crate) fn orig_ident(&self) -> &syn::Ident {
        match self {
            Self::Field(me) => &me.ident,
            Self::FinishFn(me) | Self::StartFn(me) => &me.ident,
            Self::Named(me) => &me.name.orig,
            Self::Skip(me) => &me.ident,
        }
    }

    pub(crate) fn as_named(&self) -> Option<&NamedMember> {
        match self {
            Self::Named(me) => Some(me),
            _ => None,
        }
    }

    pub(crate) fn as_field(&self) -> Option<&CustomField> {
        match self {
            Self::Field(me) => Some(me),
            _ => None,
        }
    }

    pub(crate) fn as_start_fn(&self) -> Option<&PosFnMember> {
        match self {
            Self::StartFn(me) => Some(me),
            _ => None,
        }
    }

    pub(crate) fn as_finish_fn(&self) -> Option<&PosFnMember> {
        match self {
            Self::FinishFn(me) => Some(me),
            _ => None,
        }
    }
}

impl PosFnMember {
    fn new(
        origin: MemberOrigin,
        member: RawMember<'_>,
        on: &[OnConfig],
        config: MemberConfig,
    ) -> Result<Self> {
        let RawMember {
            attrs: _,
            ident,
            ty,
            span: _,
        } = member;

        let mut me = Self {
            origin,
            ident,
            ty,
            config,
        };

        me.merge_config_into(on)?;

        Ok(me)
    }
}

impl CustomField {
    fn new(member: RawMember<'_>, init: Option<syn::Expr>) -> Result<Self> {
        if member.ident.to_string().starts_with("__") {
            bail!(
                &member.ident,
                "field names starting with `__` are reserved for `bon`'s internal use; \
                please, select a different name",
            );
        }

        Ok(Self {
            ident: member.ident,
            norm_ty: member.ty.norm,
            init,
        })
    }
}
