use super::config::MemberConfig;
use super::{config, MemberOrigin};
use crate::builder::builder_gen::member::config::SettersFnsConfig;
use crate::builder::builder_gen::top_level_config::OnConfig;
use crate::normalization::SyntaxVariant;
use crate::parsing::{ItemSigConfig, SpannedKey};
use crate::util::prelude::*;
use proc_macro2::TokenTree;

#[derive(Debug)]
pub(crate) struct MemberName {
    /// Original name of the member (unchanged). It's used in the finishing
    /// function of the builder to create a variable for each member.
    pub(crate) orig: syn::Ident,

    /// `snake_case` version of the member name. By default it's the `orig` name
    /// itself with the `_` prefix stripped. Otherwise the user can override it
    /// via `#[builder(name = custom_name)]`
    pub(crate) snake: syn::Ident,

    /// `snake_case` version of the member name as a string without the `r#` prefix
    /// (if there is any in the `snake` representation). It's computed and
    /// stored separately to avoid recomputing it multiple times. It's used
    /// to derive names for other identifiers that are based on the `snake_case` name.
    pub(crate) snake_raw_str: String,

    /// `PascalCase` version of the member name. It's always computed as the
    /// `snake` variant converted to `PascalCase`. The user doesn't have the
    /// granular control over this name. Users can only specify the snake case
    /// version of the name, and the pascal case is derived from it.
    pub(crate) pascal: syn::Ident,

    /// `PascalCase` version of the member name as a string. It's computed and
    /// stored separately to avoid recomputing it multiple times. It's guaranteed
    /// to not have the `r#` prefix because:
    ///
    /// There are no pascal case keywords in Rust except for `Self`, which
    /// is anyway not allowed even as a raw identifier:
    /// <https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094>
    pub(crate) pascal_str: String,
}

impl MemberName {
    pub(crate) fn new(orig: syn::Ident, config: &MemberConfig) -> Self {
        let snake = config.name.clone().unwrap_or_else(|| {
            let orig_str = orig.to_string();
            let norm = orig_str
                // Remove the leading underscore from the member name since it's used
                // to denote unused symbols in Rust. That doesn't mean the builder
                // API should expose that knowledge to the caller.
                .strip_prefix('_')
                .unwrap_or(&orig_str);

            // Preserve the original identifier span to make IDE's "go to definition" work correctly
            // and make error messages point to the correct place.
            syn::Ident::new_maybe_raw(norm, orig.span())
        });

        let pascal = snake.snake_to_pascal_case();

        Self {
            orig,
            snake_raw_str: snake.raw_name(),
            snake,
            pascal_str: pascal.to_string(),
            pascal,
        }
    }
}

/// Regular member for which the builder should have setter methods
#[derive(Debug)]
pub(crate) struct NamedMember {
    /// Specifies what syntax the member comes from.
    pub(crate) origin: MemberOrigin,

    /// Index of the member relative to other named members. The index is 0-based.
    pub(crate) index: syn::Index,

    /// Name of the member is used to generate names for the setters, names for
    /// the associated types and type aliases in the builder state, etc.
    pub(crate) name: MemberName,

    /// Doc comments on top of the original syntax. These are copied to the setters
    /// unless there are overrides for them.
    pub(crate) docs: Vec<syn::Attribute>,

    /// Type of the member has to be known to generate the types for fields in
    /// the builder, signatures of the setter methods, etc.
    pub(crate) ty: SyntaxVariant<Box<syn::Type>>,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) config: MemberConfig,

    /// Preserve a span the documentation for the member's methods should link to.
    pub(crate) span: Span,
}

impl NamedMember {
    pub(super) fn validate(&self) -> Result {
        if let Some(default) = &self.config.default {
            if self.is_special_option_ty() {
                bail!(
                    &default.key,
                    "`Option<_>` already implies a default of `None`, \
                    so explicit #[builder(default)] is redundant",
                );
            }
        }

        let member_docs_not_copied = self
            .config
            .setters
            .as_ref()
            .map(|setters| {
                if setters.doc.content.is_some() {
                    return true;
                }

                let SettersFnsConfig { some_fn, option_fn } = &setters.fns;
                matches!(
                    (some_fn.as_deref(), option_fn.as_deref()),
                    (
                        Some(ItemSigConfig { docs: Some(_), .. }),
                        Some(ItemSigConfig { docs: Some(_), .. })
                    )
                )
            })
            .unwrap_or(false);

        if !member_docs_not_copied {
            crate::parsing::reject_self_mentions_in_docs(
                "builder struct's impl block",
                &self.docs,
            )?;
        }

        self.validate_setters_config()?;

        if self.config.required.is_present() && !self.ty.norm.is_option() {
            bail!(
                &self.config.required.span(),
                "`#[builder(required)]` can only be applied to members of \
                type `Option<T>` to disable their special handling",
            );
        }

        Ok(())
    }

    fn validate_setters_config(&self) -> Result {
        let setters = match &self.config.setters {
            Some(setters) => setters,
            None => return Ok(()),
        };

        if self.is_required() {
            let SettersFnsConfig { some_fn, option_fn } = &setters.fns;

            let unexpected_setter = option_fn.as_ref().or(some_fn.as_ref());

            if let Some(setter) = unexpected_setter {
                bail!(
                    &setter.key,
                    "`{}` setter function applies only to members with `#[builder(default)]` \
                     or members of `Option<T>` type (if #[builder(required)] is not set)",
                    setter.key
                );
            }
        }

        if let SettersFnsConfig {
            some_fn: Some(some_fn),
            option_fn: Some(option_fn),
        } = &setters.fns
        {
            let setter_fns = &[some_fn, option_fn];

            Self::validate_unused_setters_cfg(setter_fns, &setters.name, |config| &config.name)?;
            Self::validate_unused_setters_cfg(setter_fns, &setters.vis, |config| &config.vis)?;
            Self::validate_unused_setters_cfg(setter_fns, &setters.doc.content, |config| {
                &config.docs
            })?;
        }

        Ok(())
    }

    fn validate_unused_setters_cfg<T>(
        overrides: &[&SpannedKey<ItemSigConfig>],
        config: &Option<SpannedKey<T>>,
        get_val: impl Fn(&ItemSigConfig) -> &Option<SpannedKey<T>>,
    ) -> Result {
        let config = match config {
            Some(config) => config,
            None => return Ok(()),
        };

        let overrides_values = overrides
            .iter()
            .copied()
            .map(|over| get_val(&over.value).as_ref());

        if !overrides_values.clone().all(|over| over.is_some()) {
            return Ok(());
        }

        let setters = overrides
            .iter()
            .map(|over| format!("`{}`", over.key))
            .join(", ");

        bail!(
            &config.key,
            "this `{name}` configuration is unused because all of the \
             {setters} setters contain a `{name}` override",
            name = config.key,
        );
    }

    /// Returns `true` if this member is of `Option<_>` type, but returns `false`
    /// if `#[builder(required)]` is set.
    pub(crate) fn is_special_option_ty(&self) -> bool {
        !self.config.required.is_present() && self.ty.norm.is_option()
    }

    /// Returns `false` if the member has a default value. It means this member
    /// is required to be set before building can be finished.
    pub(crate) fn is_required(&self) -> bool {
        self.config.default.is_none() && !self.is_special_option_ty()
    }

    /// A stateful member is the one that has a corresponding associated type in
    /// the builder's type state trait. This is used to track the fact that the
    /// member was set or not. This is necessary to make sure all members without
    /// default values are set before building can be finished.
    pub(crate) fn is_stateful(&self) -> bool {
        self.is_required() || !self.config.overwritable.is_present()
    }

    /// Returns the normalized type of the member stripping the `Option<_>`
    /// wrapper if it's present unless `#[builder(required)]` is set.
    pub(crate) fn underlying_norm_ty(&self) -> &syn::Type {
        self.underlying_ty(&self.ty.norm)
    }

    /// Returns the original type of the member stripping the `Option<_>`
    /// wrapper if it's present unless `#[builder(required)]` is set.
    pub(crate) fn underlying_orig_ty(&self) -> &syn::Type {
        self.underlying_ty(&self.ty.orig)
    }

    fn underlying_ty<'m>(&'m self, ty: &'m syn::Type) -> &'m syn::Type {
        if self.config.required.is_present() || self.config.default.is_some() {
            ty
        } else {
            ty.option_type_param().unwrap_or(ty)
        }
    }

    pub(crate) fn is(&self, other: &Self) -> bool {
        self.index == other.index
    }

    pub(crate) fn merge_on_config(&mut self, on: &[OnConfig]) -> Result {
        // This is a temporary hack. We only allow `on(_, required)` as the
        // first `on(...)` clause. Instead we should implement the extended design:
        // https://github.com/elastio/bon/issues/152
        if let Some(on) = on.first().filter(|on| on.required.is_present()) {
            if self.is_special_option_ty() {
                self.config.required = on.required;
            }
        }

        self.merge_config_into(on)?;
        self.merge_setters_doc_skip(on)?;

        // FIXME: refactor this to make it more consistent with `into`
        // and allow for non-boolean flags in `OnConfig`. E.g. add support
        // for `with = closure` to `on` as well.
        self.config.overwritable = config::EvalBlanketFlagParam {
            on,
            param_name: config::BlanketParamName::Overwritable,
            member_config: &self.config,
            scrutinee: self.underlying_norm_ty(),
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }

    fn merge_setters_doc_skip(&mut self, on: &[OnConfig]) -> Result {
        let skip = config::EvalBlanketFlagParam {
            on,
            param_name: config::BlanketParamName::SettersDocDefaultSkip,
            member_config: &self.config,
            scrutinee: self.underlying_norm_ty(),
            origin: self.origin,
        }
        .eval()?;

        let setters = self.config.setters.get_or_insert_with(Default::default);
        let default = setters.doc.default.get_or_insert_with(Default::default);

        default.skip = skip;

        Ok(())
    }

    /// Respan the tokens with the member's span. This is important to make
    /// rustdoc's source links point to the original member's location.
    pub(crate) fn respan(&self, tokens: TokenStream) -> impl Iterator<Item = TokenTree> {
        let span = self.span;
        tokens.into_iter().map(move |mut token| {
            token.set_span(span);
            token
        })
    }
}
