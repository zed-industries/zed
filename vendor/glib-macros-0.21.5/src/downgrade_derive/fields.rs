// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident, Type};

use crate::utils::crate_ident_new;

/// Parts needed to derive Downgrade and Upgrade implementation.
pub struct DowngradeStructParts {
    /// Inner part of weak type declaration
    pub weak_fields: TokenStream,
    /// Term needed to finish declaration. It is usually blank but is `;` for tuple structs.
    pub end_of_struct: TokenStream,
    /// Destructuring pattern
    pub destruct: TokenStream,
    /// Downgrade code
    pub downgrade: TokenStream,
    /// Upgrade code
    pub upgrade: TokenStream,
}

/// This function generates parts needed to derive Downgrade and Upgrade
/// implementations.
///
/// # Example
///
/// Let's assume following types are declared.
///
/// ```rust,ignore
/// struct Unnamed(X, Y);
///
/// struct Named {
///     x: X,
///     y: Y,
/// }
///
/// enum Choice {
///     This(X, Y),
///     That { x: X, y: Y },
/// }
/// ```
///
/// ## weak_fields
///
/// For the struct `Unnamed` and for a enum's variant `Choice::This`
/// it will be `(<X as Downgrade>::Weak, <Y as Downgrade>::Weak)`.
/// For the struct `Named` and for a enum's variant `Choice::That`
/// it will be `{ x: <X as Downgrade>::Weak, y: <Y as Downgrade>::Weak, }`.
///
/// ## end_of_struct
///
/// It is a semicolon (`;`) for an `Unnamed` and is blank for the rest.
///
/// ## destruct
///
/// For the struct `Unnamed` and for a enum's variant `Choice::This`
/// it will be `(ref _0, ref _1)`.
/// For the struct `Named` and for a enum's variant `Choice::That`
/// it will be `{ ref x, ref y }`.
/// So it can be used as a destructuring pattern for values of both types,
/// strong and weak.
///
/// ```rust,ignore
/// let Unnamed (ref _0, ref _1) = <expression>;
/// let Named { ref x, ref y } = <expression>;
///
/// match <expression> {
///     Choice::This (ref _0, ref _1) => ... ,
///     Choice::That { ref x, ref y } => ... ,
/// }
/// ```
///
/// # downgrade
///
/// ```rust,ignore
/// (
///     glib::clone::Downgrade::downgrade(_0),
///     glib::clone::Downgrade::downgrade(_1),
/// )
///
/// {
///     x: glib::clone::Downgrade::downgrade(x),
///     y: glib::clone::Downgrade::downgrade(y),
/// }
/// ```
///
/// # upgrade
///
/// ```rust,ignore
/// (
///     glib::clone::Upgrade::upgrade(_0)?,
///     glib::clone::Upgrade::upgrade(_1)?,
/// )
///
/// {
///     x: glib::clone::Upgrade::upgrade(x)?,
///     y: glib::clone::Upgrade::upgrade(y)?,
/// }
/// ```
pub fn derive_downgrade_fields(fields: syn::Fields) -> DowngradeStructParts {
    let glib = crate_ident_new();
    match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let fields: Vec<Type> = unnamed
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| field.ty)
                .collect();

            let weak_fields: Vec<_> = fields
                .iter()
                .map(|ty| {
                    quote! {
                        <#ty as #glib::clone::Downgrade>::Weak
                    }
                })
                .collect();

            let field_ident: Vec<Ident> =
                (0..fields.len()).map(|i| format_ident!("_{}", i)).collect();

            DowngradeStructParts {
                weak_fields: quote! {
                    (#(
                        #weak_fields
                    ),*)
                },
                end_of_struct: quote!(;),
                destruct: quote! {
                    (#(
                        ref #field_ident
                    ),*)
                },
                downgrade: quote! {
                    (#(
                        #glib::clone::Downgrade::downgrade(#field_ident)
                    ),*)
                },
                upgrade: quote! {
                    (#(
                        #glib::clone::Upgrade::upgrade(#field_ident)?
                    ),*)
                },
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields: Vec<(Ident, Type)> = named
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| (field.ident.expect("Field ident is specified"), field.ty))
                .collect();

            let weak_fields: Vec<_> = fields
                .iter()
                .map(|(ident, ty)| {
                    quote! {
                        #ident: <#ty as #glib::clone::Downgrade>::Weak
                    }
                })
                .collect();

            let field_ident: Vec<_> = fields.iter().map(|(ident, _ty)| ident).collect();

            DowngradeStructParts {
                weak_fields: quote! {
                    {#(
                        #weak_fields
                    ),*}
                },
                end_of_struct: quote!(),
                destruct: quote! {
                    {#(
                        ref #field_ident
                    ),*}
                },
                downgrade: quote! {
                    {#(
                        #field_ident: #glib::clone::Downgrade::downgrade(#field_ident)
                    ),*}
                },
                upgrade: quote! {
                    {#(
                        #field_ident: #glib::clone::Upgrade::upgrade(#field_ident)?
                    ),*}
                },
            }
        }
        Fields::Unit => DowngradeStructParts {
            weak_fields: quote! {},
            end_of_struct: quote! { ; },
            destruct: quote! {},
            downgrade: quote! {},
            upgrade: quote! {},
        },
    }
}
