//! Specialised configuration for `DocumentedFields` and `DocumentedFieldsOpt`.

use convert_case::Case;
use syn::Expr;

/// Defines how to rename a particular field.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RenameMode {
    /// Use the original name, converted to another case.
    ToCase(Case<'static>),
    /// Use a custom name.
    Custom(String),
}

#[cfg_attr(feature = "customise", optfield::optfield(
    pub DeriveFieldsBaseCustomisations,
    attrs = (derive(Clone, Debug, Default, PartialEq, Eq)),
    merge_fn = pub apply_base_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    Specialised variant for the type base of `DocumentedFields` and `DocumentedFieldsOpt`.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[cfg_attr(feature = "customise", optfield::optfield(
    pub DeriveFieldsCustomisations,
    attrs = (derive(Clone, Debug, Default, PartialEq, Eq)),
    merge_fn = pub apply_field_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    Specialised variant for each field of `DocumentedFields` and `DocumentedFieldsOpt`.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
/// Configurable options for each field via helper attributes.
///
/// Initial values are set to default.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeriveFieldsConfig {
    // optfield does not rewrap `Option` by default, which is the desired behavior
    // see https://docs.rs/optfield/latest/optfield/#rewrapping-option-fields
    pub rename_mode: Option<RenameMode>,
    pub default_value: Option<Expr>,
    pub trim: bool,
}
impl Default for DeriveFieldsConfig {
    fn default() -> Self {
        Self {
            rename_mode: None,
            default_value: None,
            trim: true,
        }
    }
}

#[cfg(feature = "customise")]
mod customise {
    use crate::config::{
        customise_core::{ConfigOption, ConfigOptionData},
        derive_fields::{
            DeriveFieldsBaseCustomisations, DeriveFieldsConfig, DeriveFieldsCustomisations,
            RenameMode,
        },
    };

    impl DeriveFieldsConfig {
        /// Return a new instance of this config with base customisations applied.
        pub fn with_base_customisations(
            &self,
            customisations: DeriveFieldsBaseCustomisations,
        ) -> Self {
            let mut new = self.clone();
            new.apply_base_customisations(customisations);
            new
        }

        /// Return a new instance of this config with field customisations applied.
        pub fn with_field_customisations(
            &self,
            customisations: DeriveFieldsCustomisations,
        ) -> Self {
            let mut new = self.clone();
            new.apply_field_customisations(customisations);
            new
        }
    }

    impl TryFrom<Vec<ConfigOption>> for DeriveFieldsBaseCustomisations {
        type Error = syn::Error;

        /// Duplicate option rejection should be handled upstream.
        fn try_from(opts: Vec<ConfigOption>) -> Result<Self, Self::Error> {
            use ConfigOptionData as Data;

            let mut config = Self::default();
            for opt in opts {
                match opt.data {
                    Data::Vis(..) | Data::Rename(..) => Err(syn::Error::new(
                        opt.span,
                        "This config option is not applicable here",
                    ))?,
                    Data::RenameAll(case) => {
                        config.rename_mode.replace(RenameMode::ToCase(case.value()));
                    }
                    Data::Default(expr) => {
                        config.default_value.replace(expr);
                    }
                    Data::Trim(trim) => {
                        config.trim.replace(trim.value());
                    }
                }
            }
            Ok(config)
        }
    }

    impl TryFrom<Vec<ConfigOption>> for DeriveFieldsCustomisations {
        type Error = syn::Error;

        /// Duplicate option rejection should be handled upstream.
        fn try_from(opts: Vec<ConfigOption>) -> Result<Self, Self::Error> {
            use ConfigOptionData as Data;

            let mut config = Self::default();
            for opt in opts {
                match opt.data {
                    Data::Vis(..) => Err(syn::Error::new(
                        opt.span,
                        "This config option is not applicable here",
                    ))?,
                    Data::RenameAll(case) => {
                        // `rename` always has priority over `rename_all`
                        if !matches!(config.rename_mode, Some(RenameMode::Custom(_))) {
                            config.rename_mode.replace(RenameMode::ToCase(case.value()));
                        }
                    }
                    Data::Rename(name) => {
                        config.rename_mode.replace(RenameMode::Custom(name.value()));
                    }
                    Data::Default(expr) => {
                        config.default_value.replace(expr);
                    }
                    Data::Trim(trim) => {
                        config.trim.replace(trim.value());
                    }
                }
            }
            Ok(config)
        }
    }
}
