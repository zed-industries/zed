//! Generic configuration for derive macros.
//!
//! If a macro needs specialised configuration, this file can be used as a
//! starting template.

use syn::Expr;

/// Configurable options for derive macros via helper attributes.
///
/// Initial values are set to default.
#[cfg_attr(feature = "customise", optfield::optfield(
    pub DeriveCustomisations,
    attrs = add(derive(Default)),
    merge_fn = pub apply_customisations,
    doc = "Parsed user-defined customisations of configurable options.\n\
    \n\
    Expected parse stream format: `<KW> = <VAL>, <KW> = <VAL>, ...`"
))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeriveConfig {
    // optfield does not rewrap `Option` by default, which is the desired behavior
    // see https://docs.rs/optfield/latest/optfield/#rewrapping-option-fields
    pub default_value: Option<Expr>,
    pub trim: bool,
}
impl Default for DeriveConfig {
    fn default() -> Self {
        Self { default_value: None, trim: true }
    }
}

#[cfg(feature = "customise")]
mod customise {
    use crate::config::{
        customise_core::{ConfigOption, ConfigOptionData},
        derive::{DeriveConfig, DeriveCustomisations},
    };

    impl DeriveConfig {
        /// Return a new instance of this config with customisations applied.
        pub fn with_customisations(&self, customisations: DeriveCustomisations) -> Self {
            let mut new = self.clone();
            new.apply_customisations(customisations);
            new
        }
    }

    impl TryFrom<Vec<ConfigOption>> for DeriveCustomisations {
        type Error = syn::Error;

        /// Duplicate option rejection should be handled upstream.
        fn try_from(opts: Vec<ConfigOption>) -> Result<Self, Self::Error> {
            use ConfigOptionData as Data;

            let mut config = Self::default();
            for opt in opts {
                match opt.data {
                    Data::Vis(..) | Data::RenameAll(..) | Data::Rename(..) => Err(
                        syn::Error::new(opt.span, "This config option is not applicable here"),
                    )?,
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
