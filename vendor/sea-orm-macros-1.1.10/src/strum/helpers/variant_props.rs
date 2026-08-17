use std::default::Default;
use syn::{Ident, LitStr, Variant};

use super::metadata::{kw, VariantExt, VariantMeta};
use super::occurrence_error;

pub trait HasStrumVariantProperties {
    fn get_variant_properties(&self) -> syn::Result<StrumVariantProperties>;
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct StrumVariantProperties {
    pub disabled: Option<kw::disabled>,
    pub default: Option<kw::default>,
    pub ascii_case_insensitive: Option<bool>,
    pub message: Option<LitStr>,
    pub detailed_message: Option<LitStr>,
    pub documentation: Vec<LitStr>,
    pub string_props: Vec<(LitStr, LitStr)>,
    serialize: Vec<LitStr>,
    to_string: Option<LitStr>,
    ident: Option<Ident>,
}

impl HasStrumVariantProperties for Variant {
    fn get_variant_properties(&self) -> syn::Result<StrumVariantProperties> {
        let mut output = StrumVariantProperties {
            ident: Some(self.ident.clone()),
            ..Default::default()
        };

        let mut message_kw = None;
        let mut detailed_message_kw = None;
        let mut to_string_kw = None;
        let mut disabled_kw = None;
        let mut default_kw = None;
        let mut ascii_case_insensitive_kw = None;
        for meta in self.get_metadata()? {
            match meta {
                VariantMeta::Message { value, kw } => {
                    if let Some(fst_kw) = message_kw {
                        return Err(occurrence_error(fst_kw, kw, "message"));
                    }

                    message_kw = Some(kw);
                    output.message = Some(value);
                }
                VariantMeta::DetailedMessage { value, kw } => {
                    if let Some(fst_kw) = detailed_message_kw {
                        return Err(occurrence_error(fst_kw, kw, "detailed_message"));
                    }

                    detailed_message_kw = Some(kw);
                    output.detailed_message = Some(value);
                }
                VariantMeta::Documentation { value } => {
                    output.documentation.push(value);
                }
                VariantMeta::Serialize { value, .. } => {
                    output.serialize.push(value);
                }
                VariantMeta::ToString { value, kw } => {
                    if let Some(fst_kw) = to_string_kw {
                        return Err(occurrence_error(fst_kw, kw, "to_string"));
                    }

                    to_string_kw = Some(kw);
                    output.to_string = Some(value);
                }
                VariantMeta::Disabled(kw) => {
                    if let Some(fst_kw) = disabled_kw {
                        return Err(occurrence_error(fst_kw, kw, "disabled"));
                    }

                    disabled_kw = Some(kw);
                    output.disabled = Some(kw);
                }
                VariantMeta::Default(kw) => {
                    if let Some(fst_kw) = default_kw {
                        return Err(occurrence_error(fst_kw, kw, "default"));
                    }

                    default_kw = Some(kw);
                    output.default = Some(kw);
                }
                VariantMeta::AsciiCaseInsensitive { kw, value } => {
                    if let Some(fst_kw) = ascii_case_insensitive_kw {
                        return Err(occurrence_error(fst_kw, kw, "ascii_case_insensitive"));
                    }

                    ascii_case_insensitive_kw = Some(kw);
                    output.ascii_case_insensitive = Some(value);
                }
                VariantMeta::Props { props, .. } => {
                    output.string_props.extend(props);
                }
            }
        }

        Ok(output)
    }
}
