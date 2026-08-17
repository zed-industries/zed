#[cfg(feature = "seaography")]
mod private {
    use heck::ToLowerCamelCase;
    use proc_macro2::{Ident, Span, TokenStream};
    use proc_macro_crate::{crate_name, FoundCrate};
    use quote::{quote, quote_spanned};

    use crate::derives::attributes::related_attr;

    enum Error {
        InputNotEnum,
        InvalidEntityPath,
        Syn(syn::Error),
    }

    struct DeriveRelatedEntity {
        entity_ident: TokenStream,
        ident: syn::Ident,
        variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
    }

    impl DeriveRelatedEntity {
        fn new(input: syn::DeriveInput) -> Result<Self, Error> {
            let sea_attr = related_attr::SeaOrm::try_from_attributes(&input.attrs)
                .map_err(Error::Syn)?
                .unwrap_or_default();

            let ident = input.ident;
            let entity_ident = match sea_attr.entity.as_ref().map(Self::parse_lit_string) {
                Some(entity_ident) => entity_ident.map_err(|_| Error::InvalidEntityPath)?,
                None => quote! { Entity },
            };

            let variants = match input.data {
                syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
                _ => return Err(Error::InputNotEnum),
            };

            Ok(DeriveRelatedEntity {
                entity_ident,
                ident,
                variants,
            })
        }

        fn expand(&self) -> syn::Result<TokenStream> {
            let ident = &self.ident;
            let entity_ident = &self.entity_ident;

            let variant_implementations: Vec<TokenStream> = self
                .variants
                .iter()
                .map(|variant| {
                    let attr = related_attr::SeaOrm::from_attributes(&variant.attrs)?;

                    let enum_name = &variant.ident;

                    let target_entity = attr
                        .entity
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'entity'")
                        })??;

                    let def = match attr.def {
                        Some(def) => Some(Self::parse_lit_string(&def).map_err(|_| {
                            syn::Error::new_spanned(variant, "Missing value for 'def'")
                        })?),
                        None => None,
                    };

                    let name = enum_name.to_string().to_lower_camel_case();

                    if let Some(def) = def {
                        Result::<_, syn::Error>::Ok(quote! {
                            Self::#enum_name => builder.get_relation::<#entity_ident, #target_entity>(#name, #def)
                        })
                    } else {
                        Result::<_, syn::Error>::Ok(quote! {
                            Self::#enum_name => via_builder.get_relation::<#entity_ident, #target_entity>(#name)
                        })
                    }

                })
                .collect::<Result<Vec<_>, _>>()?;

            // Get the path of the `async-graphql` on the application's Cargo.toml
            let async_graphql_crate = match crate_name("async-graphql") {
                // if found, use application's `async-graphql`
                Ok(FoundCrate::Name(name)) => {
                    let ident = Ident::new(&name, Span::call_site());
                    quote! { #ident }
                }
                Ok(FoundCrate::Itself) => quote! { async_graphql },
                // if not, then use the `async-graphql` re-exported by `seaography`
                Err(_) => quote! { seaography::async_graphql },
            };

            Ok(quote! {
                impl seaography::RelationBuilder for #ident {
                    fn get_relation(&self, context: & 'static seaography::BuilderContext) -> #async_graphql_crate::dynamic::Field {
                        let builder = seaography::EntityObjectRelationBuilder { context };
                        let via_builder = seaography::EntityObjectViaRelationBuilder { context };
                        match self {
                            #(#variant_implementations,)*
                            _ => panic!("No relations for this entity"),
                        }
                    }

                }
            })
        }

        fn parse_lit_string(lit: &syn::Lit) -> syn::Result<TokenStream> {
            match lit {
                syn::Lit::Str(lit_str) => lit_str
                    .value()
                    .parse()
                    .map_err(|_| syn::Error::new_spanned(lit, "attribute not valid")),
                _ => Err(syn::Error::new_spanned(lit, "attribute must be a string")),
            }
        }
    }

    /// Method to derive a Related enumeration
    pub fn expand_derive_related_entity(input: syn::DeriveInput) -> syn::Result<TokenStream> {
        let ident_span = input.ident.span();

        match DeriveRelatedEntity::new(input) {
            Ok(model) => model.expand(),
            Err(Error::InputNotEnum) => Ok(quote_spanned! {
                ident_span => compile_error!("you can only derive DeriveRelation on enums");
            }),
            Err(Error::InvalidEntityPath) => Ok(quote_spanned! {
                ident_span => compile_error!("invalid attribute value for 'entity'");
            }),
            Err(Error::Syn(err)) => Err(err),
        }
    }
}

#[cfg(not(feature = "seaography"))]
mod private {
    use proc_macro2::TokenStream;

    pub fn expand_derive_related_entity(_: syn::DeriveInput) -> syn::Result<TokenStream> {
        Ok(TokenStream::new())
    }
}

pub use private::*;
