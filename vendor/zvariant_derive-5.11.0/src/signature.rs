use std::str::FromStr;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{Error, parse::Parse};
use zvariant_utils::signature::Signature;

/// Expand the `signature!` macro implementation.
///
/// Takes a string literal signature and converts it to compile-time tokens
/// representing a const `Signature`.
pub fn expand_signature_macro(input: TokenStream) -> Result<TokenStream, Error> {
    let SignatureInput {
        literal: signature_str,
    } = syn::parse2(input)?;

    let signature_string = signature_str.to_string();
    let signature_string = signature_string.trim_matches('"');

    let signature = match signature_string {
        "dict" => Signature::dict(Signature::Str, Signature::Variant),
        s => Signature::from_str(s).map_err(|e| Error::new(signature_str.span(), e))?,
    };

    let signature_tokens = signature_to_tokens(&signature);

    Ok(signature_tokens)
}

/// Input type for the signature macro.
struct SignatureInput {
    literal: Literal,
}

impl Parse for SignatureInput {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        Ok(SignatureInput {
            literal: input.parse()?,
        })
    }
}

/// Converts a parsed `Signature` to compile-time token representation.
///
/// This function generates the Rust tokens that will construct the signature
/// at compile time. Used by both the signature! macro and the Type derive macro.
pub fn signature_to_tokens(signature: &Signature) -> TokenStream {
    signature_to_tokens_with_crate(signature, &quote! { ::zvariant })
}

/// Converts a parsed `Signature` to compile-time token representation with a custom crate path.
///
/// This function generates the Rust tokens that will construct the signature
/// at compile time, using the provided crate path for zvariant.
pub fn signature_to_tokens_with_crate(signature: &Signature, zv: &TokenStream) -> TokenStream {
    match signature {
        Signature::Unit => quote! { #zv::Signature::Unit },
        Signature::Bool => quote! { #zv::Signature::Bool },
        Signature::U8 => quote! { #zv::Signature::U8 },
        Signature::I16 => quote! { #zv::Signature::I16 },
        Signature::U16 => quote! { #zv::Signature::U16 },
        Signature::I32 => quote! { #zv::Signature::I32 },
        Signature::U32 => quote! { #zv::Signature::U32 },
        Signature::I64 => quote! { #zv::Signature::I64 },
        Signature::U64 => quote! { #zv::Signature::U64 },
        Signature::F64 => quote! { #zv::Signature::F64 },
        Signature::Str => quote! { #zv::Signature::Str },
        Signature::Signature => quote! { #zv::Signature::Signature },
        Signature::ObjectPath => quote! { #zv::Signature::ObjectPath },
        Signature::Variant => quote! { #zv::Signature::Variant },
        #[cfg(unix)]
        Signature::Fd => quote! { #zv::Signature::Fd },
        Signature::Array(child) => {
            let signature = signature_to_tokens_with_crate(child.signature(), zv);
            quote! {
                #zv::Signature::Array(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
        Signature::Dict { key, value } => {
            let key_sig = signature_to_tokens_with_crate(key.signature(), zv);
            let value_sig = signature_to_tokens_with_crate(value.signature(), zv);
            quote! {
                #zv::Signature::Dict {
                    key: #zv::signature::Child::Static {
                        child: &#key_sig,
                    },
                    value: #zv::signature::Child::Static {
                        child: &#value_sig,
                    },
                }
            }
        }
        Signature::Structure(fields) => {
            let fields = fields.iter().map(|f| signature_to_tokens_with_crate(f, zv));
            quote! {
                #zv::Signature::Structure(#zv::signature::Fields::Static {
                    fields: &[#(&#fields),*],
                })
            }
        }
        #[cfg(feature = "gvariant")]
        Signature::Maybe(child) => {
            let signature = signature_to_tokens_with_crate(child.signature(), zv);
            quote! {
                #zv::Signature::Maybe(#zv::signature::Child::Static {
                    child: &#signature,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_to_tokens_with_crate_uses_custom_path() {
        let custom_path = quote! { ::zbus::zvariant };
        let sig = Signature::Str;

        let tokens = signature_to_tokens_with_crate(&sig, &custom_path).to_string();

        assert!(
            tokens.contains("zbus"),
            "Expected custom path in output: {}",
            tokens
        );
    }

    #[test]
    fn signature_to_tokens_with_crate_uses_custom_path_for_complex_types() {
        let custom_path = quote! { ::zbus::zvariant };

        // Dict signature - has multiple path references
        let dict_sig = Signature::from_str("a{sv}").unwrap();
        let tokens = signature_to_tokens_with_crate(&dict_sig, &custom_path).to_string();

        // All occurrences should use the custom path
        assert!(
            !tokens.contains(":: zvariant ::") || tokens.contains(":: zbus :: zvariant ::"),
            "Found bare ::zvariant without ::zbus prefix: {}",
            tokens
        );
        assert!(
            tokens.contains(":: zbus :: zvariant ::"),
            "Expected custom path in struct output: {}",
            tokens
        );

        // Structure signature - has multiple path references
        let struct_sig = Signature::from_str("(su)").unwrap();
        let tokens = signature_to_tokens_with_crate(&struct_sig, &custom_path).to_string();

        // All occurrences should use the custom path
        assert!(
            tokens.contains("zbus"),
            "Expected custom path in struct output: {}",
            tokens
        );
    }
}
