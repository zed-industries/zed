use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, parse_macro_input};

/// Derives [`feature_flags::FeatureFlagValue`] for a unit-only enum.
///
/// Exactly one variant must be marked with `#[default]`. The default variant
/// is the one returned when the feature flag is announced by the server,
/// enabled for all users, or enabled by the staff rule — it's the "on"
/// value, and also the fallback for `from_wire`.
///
/// The generated impl derives:
///
/// * `all_variants` — every variant, in source order.
/// * `override_key` — the variant name, lower-cased with dashes between
///   PascalCase word boundaries (e.g. `NewWorktree` → `"new-worktree"`).
/// * `label` — the variant name with PascalCase boundaries expanded to
///   spaces (e.g. `NewWorktree` → `"New Worktree"`).
/// * `from_wire` — always returns the default variant, since today the
///   server wire format is just presence and does not carry a variant.
///
/// ## Example
///
/// ```ignore
/// #[derive(Clone, Copy, PartialEq, Eq, Debug, EnumFeatureFlag)]
/// enum Intensity {
///     #[default]
///     Low,
///     High,
/// }
/// ```
// `attributes(default)` lets users write `#[default]` on a variant even when
// they're not also deriving `Default`. If `#[derive(Default)]` is present in
// the same list, it reuses the same attribute — there's no conflict, because
// helper attributes aren't consumed.
#[proc_macro_derive(EnumFeatureFlag, attributes(default))]
pub fn derive_enum_feature_flag(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand(&input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "EnumFeatureFlag can only be derived for enums",
        ));
    };

    if data.variants.is_empty() {
        return Err(syn::Error::new_spanned(
            input,
            "EnumFeatureFlag requires at least one variant",
        ));
    }

    let mut default_ident: Option<&Ident> = None;
    let mut variant_idents: Vec<&Ident> = Vec::new();

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "EnumFeatureFlag only supports unit variants (no fields)",
            ));
        }
        if has_default_attr(variant) {
            if default_ident.is_some() {
                return Err(syn::Error::new_spanned(
                    variant,
                    "only one variant may be marked with #[default]",
                ));
            }
            default_ident = Some(&variant.ident);
        }
        variant_idents.push(&variant.ident);
    }

    let Some(default_ident) = default_ident else {
        return Err(syn::Error::new_spanned(
            input,
            "EnumFeatureFlag requires exactly one variant to be marked with #[default]",
        ));
    };

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let override_key_arms = variant_idents.iter().map(|variant| {
        let key = LitStr::new(&to_kebab_case(&variant.to_string()), Span::call_site());
        quote! { #name::#variant => #key }
    });

    let label_arms = variant_idents.iter().map(|variant| {
        let label = LitStr::new(&to_space_separated(&variant.to_string()), Span::call_site());
        quote! { #name::#variant => #label }
    });

    let all_variants = variant_idents.iter().map(|v| quote! { #name::#v });

    Ok(quote! {
        impl #impl_generics ::std::default::Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                #name::#default_ident
            }
        }

        impl #impl_generics ::feature_flags::FeatureFlagValue for #name #ty_generics #where_clause {
            fn all_variants() -> &'static [Self] {
                &[ #( #all_variants ),* ]
            }

            fn override_key(&self) -> &'static str {
                match self {
                    #( #override_key_arms ),*
                }
            }

            fn label(&self) -> &'static str {
                match self {
                    #( #label_arms ),*
                }
            }

            fn from_wire(_: &str) -> ::std::option::Option<Self> {
                ::std::option::Option::Some(#name::#default_ident)
            }
        }
    })
}

fn has_default_attr(variant: &syn::Variant) -> bool {
    variant.attrs.iter().any(|a| a.path().is_ident("default"))
}

/// Converts a PascalCase identifier to lowercase kebab-case.
///
/// `"NewWorktree"` → `"new-worktree"`, `"Low"` → `"low"`,
/// `"HTTPServer"` → `"httpserver"` (acronyms are not split — keep variant
/// names descriptive to avoid this).
fn to_kebab_case(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 4);
    for (i, ch) in ident.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i != 0 {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

/// Converts a PascalCase identifier to space-separated word form for display.
///
/// `"NewWorktree"` → `"New Worktree"`, `"Low"` → `"Low"`.
fn to_space_separated(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 4);
    for (i, ch) in ident.chars().enumerate() {
        if ch.is_ascii_uppercase() && i != 0 {
            out.push(' ');
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_case() {
        assert_eq!(to_kebab_case("Low"), "low");
        assert_eq!(to_kebab_case("NewWorktree"), "new-worktree");
        assert_eq!(to_kebab_case("A"), "a");
    }

    #[test]
    fn space_separated() {
        assert_eq!(to_space_separated("Low"), "Low");
        assert_eq!(to_space_separated("NewWorktree"), "New Worktree");
    }
}
