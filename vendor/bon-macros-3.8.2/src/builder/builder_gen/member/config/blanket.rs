use super::MemberConfig;
use crate::builder::builder_gen::member::MemberOrigin;
use crate::builder::builder_gen::top_level_config::OnConfig;
use crate::util::prelude::*;
use std::fmt;

pub(crate) enum BlanketParamName {
    Into,
    Overwritable,
    SettersDocDefaultSkip,
}

impl fmt::Display for BlanketParamName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Into => fmt::Display::fmt(&super::ParamName::Into, f),
            Self::Overwritable => fmt::Display::fmt(&super::ParamName::Overwritable, f),
            Self::SettersDocDefaultSkip => f.write_str("setters(doc(default(skip)))"),
        }
    }
}

impl BlanketParamName {
    fn value_in_on_config(&self, cfg: &OnConfig) -> darling::util::Flag {
        match self {
            Self::Into => cfg.into,
            Self::Overwritable => cfg.overwritable,
            Self::SettersDocDefaultSkip => cfg.setters.doc.default.skip,
        }
    }

    fn value_in_member_config(&self, cfg: &MemberConfig) -> darling::util::Flag {
        match self {
            Self::Into => cfg.into,
            Self::Overwritable => cfg.overwritable,
            Self::SettersDocDefaultSkip => cfg
                .setters
                .as_ref()
                .and_then(|setters| setters.doc.default.as_ref())
                .map(|default| default.skip)
                .unwrap_or_default(),
        }
    }
}

pub(crate) struct EvalBlanketFlagParam<'a> {
    pub(crate) on: &'a [OnConfig],
    pub(crate) param_name: BlanketParamName,
    pub(crate) member_config: &'a MemberConfig,
    pub(crate) scrutinee: &'a syn::Type,
    pub(crate) origin: MemberOrigin,
}

impl EvalBlanketFlagParam<'_> {
    pub(crate) fn eval(self) -> Result<darling::util::Flag> {
        let Self {
            on,
            param_name,
            member_config,
            scrutinee,
            origin,
        } = self;

        let verdict_from_on = on
            .iter()
            .map(|params| Ok((params, scrutinee.matches(&params.type_pattern)?)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|(_, matched)| *matched)
            .map(|(params, _)| param_name.value_in_on_config(params))
            .find(darling::util::Flag::is_present);

        let value_in_member = param_name.value_in_member_config(member_config);
        let flag = match (verdict_from_on, value_in_member.is_present()) {
            (Some(_), true) => {
                bail!(
                    &value_in_member.span(),
                    "this `#[builder({param_name})]` attribute is redundant, because \
                    `{param_name}` is already implied for this member via the \
                    `#[builder(on(...))]` at the top of the {}",
                    origin.parent_construct(),
                );
            }
            (Some(flag), false) => flag,
            (None, _) => value_in_member,
        };

        Ok(flag)
    }
}
