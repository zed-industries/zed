use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use zvariant_utils::{case, def_attrs};

/// Parses the `crate` attribute value into a path.
pub fn parse_crate_path(crate_attr: Option<&str>) -> Result<Option<syn::Path>, syn::Error> {
    crate_attr.map(syn::parse_str).transpose()
}

/// Returns the path to the zvariant crate.
///
/// If a custom crate path is provided via the `crate` attribute, it will be used.
/// Otherwise, uses `proc-macro-crate` to detect the crate name.
pub fn zvariant_path(crate_path: Option<&syn::Path>) -> TokenStream {
    if let Some(path) = crate_path {
        quote! { ::#path }
    } else if let Ok(FoundCrate::Name(name)) = crate_name("zvariant") {
        let ident = format_ident!("{}", name);
        quote! { ::#ident }
    } else if let Ok(FoundCrate::Name(name)) = crate_name("zbus") {
        let ident = format_ident!("{}", name);
        quote! { ::#ident::zvariant }
    } else {
        quote! { ::zvariant }
    }
}

pub fn rename_identifier(
    ident: String,
    span: proc_macro2::Span,
    rename_attr: Option<String>,
    rename_all_attr: Option<&str>,
) -> Result<String, syn::Error> {
    if let Some(name) = rename_attr {
        Ok(name)
    } else {
        match rename_all_attr {
            Some("lowercase") => Ok(ident.to_ascii_lowercase()),
            Some("UPPERCASE") => Ok(ident.to_ascii_uppercase()),
            Some("PascalCase") => Ok(case::pascal_or_camel_case(&ident, true)),
            Some("camelCase") => Ok(case::pascal_or_camel_case(&ident, false)),
            Some("snake_case") => Ok(case::snake_or_kebab_case(&ident, true)),
            Some("kebab-case") => Ok(case::snake_or_kebab_case(&ident, false)),
            None => Ok(ident),
            Some(other) => Err(syn::Error::new(
                span,
                format!("invalid `rename_all` attribute value {other}"),
            )),
        }
    }
}

def_attrs! {
    crate zbus, zvariant;

    /// Attributes defined on structures.
    pub StructAttributes("struct") { signature str, rename_all str, deny_unknown_fields none, crate_path str };
    /// Attributes defined on fields.
    pub FieldAttributes("field") { rename str };
    /// Attributes defined on enumerations.
    pub EnumAttributes("enum") { signature str, rename_all str, crate_path str };
    /// Attributes defined on variants.
    pub VariantAttributes("variant") { rename str };
}
