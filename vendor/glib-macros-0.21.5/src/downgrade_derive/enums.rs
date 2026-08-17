// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Generics, Ident};

use super::fields::{derive_downgrade_fields, DowngradeStructParts};
use crate::utils::crate_ident_new;

/// This function derives a weak type for a given strong enum and
/// implementations of `Downgrade` and `Upgrade` traits.
///
/// # Example
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// enum Choice {
///     This(X, Y),
///     That { x: X, y: Y },
/// }
/// ```
///
/// Here is what will be derived:
///
/// ```rust,ignore
/// enum ChoiceWeak {
///     This(<X as Downgrade>::Weak, <Y as Downgrade>::Weak),
///     That {
///         x: <X as Downgrade>::Weak,
///         y: <Y as Downgrade>::Weak,
///     },
/// }
///
/// impl glib::clone::Downgrade for Choice {
///     type Weak = ChoiceWeak;
///
///     fn downgrade(&self) -> Self::Weak {
///         match self {
///             Self::This(ref _0, ref _1) => Self::Weak::This(
///                 glib::clone::Downgrade::downgrade(_0),
///                 glib::clone::Downgrade::downgrade(_1),
///             ),
///             Self::That { ref x, ref y } => Self::Weak::That(
///                 glib::clone::Downgrade::downgrade(x),
///                 glib::clone::Downgrade::downgrade(y),
///             ),
///         }
///     }
/// }
///
/// impl glib::clone::Upgrade for ChoiceWeak {
///     type Strong = Choice;
///
///     fn upgrade(&self) -> Option<Self::Strong> {
///         Some(match self {
///             Self::This(ref _0, ref _1) => Self::Strong::This(
///                 glib::clone::Upgrade::upgrade(_0)?,
///                 glib::clone::Upgrade::upgrade(_1)?,
///             ),
///             Self::That { ref x, ref y } => Self::Strong::That(
///                 glib::clone::Upgrade::upgrade(x)?,
///                 glib::clone::Upgrade::upgrade(y)?,
///             ),
///         })
///     }
/// }
/// ```
pub fn derive_downgrade_for_enum(
    ident: Ident,
    generics: Generics,
    data_enum: syn::DataEnum,
) -> TokenStream {
    let glib = crate_ident_new();
    let weak_type = format_ident!("{}Weak", ident);

    let variants: Vec<(Ident, DowngradeStructParts)> = data_enum
        .variants
        .into_iter()
        .map(|variant| (variant.ident, derive_downgrade_fields(variant.fields)))
        .collect();

    let weak_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let weak_fields = &parts.weak_fields;
            quote! {
                #ident #weak_fields
            }
        })
        .collect();

    let downgrade_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let destruct = &parts.destruct;
            let downgrade = &parts.downgrade;
            quote! {
                Self::#ident #destruct => Self::Weak::#ident #downgrade
            }
        })
        .collect();

    let upgrade_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let destruct = &parts.destruct;
            let upgrade = &parts.upgrade;
            quote! {
                Self::#ident #destruct => Self::Strong::#ident #upgrade
            }
        })
        .collect();

    let derived = quote! {
        pub enum #weak_type #generics {#(
            #weak_variants
        ),*}

        impl #generics #glib::clone::Downgrade for #ident #generics {
            type Weak = #weak_type #generics;

            fn downgrade(&self) -> Self::Weak {
                match self {#(
                    #downgrade_variants
                ),*}
            }
        }

        impl #generics #glib::clone::Upgrade for #weak_type #generics {
            type Strong = #ident #generics;

            fn upgrade(&self) -> ::core::option::Option<Self::Strong> {
                ::core::option::Option::Some(match self {#(
                    #upgrade_variants
                ),*})
            }
        }
    };

    derived.into()
}
