use heck::ToSnakeCase;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Data, DataEnum, Expr, Fields, LitStr, Variant};

/// Method to derive a Primary Key for a Model using the [PrimaryKeyTrait](sea_orm::PrimaryKeyTrait)
pub fn expand_derive_primary_key(ident: Ident, data: Data) -> syn::Result<TokenStream> {
    let variants = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => variants,
        _ => {
            return Ok(quote_spanned! {
                ident.span() => compile_error!("you can only derive DerivePrimaryKey on enums");
            })
        }
    };

    if variants.is_empty() {
        return Ok(quote_spanned! {
            ident.span() => compile_error!("Entity must have a primary key column. See <https://github.com/SeaQL/sea-orm/issues/485> for details.");
        });
    }

    let variant: Vec<TokenStream> = variants
        .iter()
        .map(|Variant { ident, fields, .. }| match fields {
            Fields::Named(_) => quote! { #ident{..} },
            Fields::Unnamed(_) => quote! { #ident(..) },
            Fields::Unit => quote! { #ident },
        })
        .collect();

    let name: Vec<TokenStream> = variants
        .iter()
        .map(|v| {
            let mut column_name = v.ident.to_string().to_snake_case();
            for attr in v.attrs.iter() {
                if !attr.path().is_ident("sea_orm") {
                    continue;
                }

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("column_name") {
                        column_name = meta.value()?.parse::<LitStr>()?.value();
                    } else {
                        // Reads the value expression to advance the parse stream.
                        // Some parameters, such as `primary_key`, do not have any value,
                        // so ignoring an error occurred here.
                        let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                    }

                    Ok(())
                })?;
            }
            Ok::<TokenStream, syn::Error>(quote! { #column_name })
        })
        .collect::<Result<_, _>>()?;

    Ok(quote!(
        #[automatically_derived]
        impl sea_orm::Iden for #ident {
            fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                write!(s, "{}", sea_orm::IdenStatic::as_str(self)).unwrap();
            }
        }

        #[automatically_derived]
        impl sea_orm::IdenStatic for #ident {
            fn as_str(&self) -> &str {
                match self {
                    #(Self::#variant => #name),*
                }
            }
        }

        #[automatically_derived]
        impl sea_orm::PrimaryKeyToColumn for #ident {
            type Column = Column;

            fn into_column(self) -> Self::Column {
                match self {
                    #(Self::#variant => Self::Column::#variant,)*
                }
            }

            fn from_column(col: Self::Column) -> Option<Self> {
                match col {
                    #(Self::Column::#variant => Some(Self::#variant),)*
                    _ => None,
                }
            }
        }
    ))
}
