use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

use super::attributes::{derive_attr, field_attr};

enum Error {
    InputNotEnum,
    Syn(syn::Error),
}

struct DeriveRelation {
    entity_ident: syn::Ident,
    ident: syn::Ident,
    variants: syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
}

impl DeriveRelation {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        let variants = match input.data {
            syn::Data::Enum(syn::DataEnum { variants, .. }) => variants,
            _ => return Err(Error::InputNotEnum),
        };

        let sea_attr = derive_attr::SeaOrm::try_from_attributes(&input.attrs)
            .map_err(Error::Syn)?
            .unwrap_or_default();

        let ident = input.ident;
        let entity_ident = sea_attr.entity.unwrap_or_else(|| format_ident!("Entity"));

        Ok(DeriveRelation {
            entity_ident,
            ident,
            variants,
        })
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        let expanded_impl_relation_trait = self.impl_relation_trait()?;

        Ok(expanded_impl_relation_trait)
    }

    fn impl_relation_trait(&self) -> syn::Result<TokenStream> {
        let ident = &self.ident;
        let entity_ident = &self.entity_ident;
        let no_relation_def_msg = format!("No RelationDef for {ident}");

        let variant_relation_defs: Vec<TokenStream> = self
            .variants
            .iter()
            .map(|variant| {
                let variant_ident = &variant.ident;
                let attr = field_attr::SeaOrm::from_attributes(&variant.attrs)?;
                let mut relation_type = quote! { error };
                let related_to = if attr.belongs_to.is_some() {
                    relation_type = quote! { belongs_to };
                    attr.belongs_to
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'belongs_to'")
                        })
                } else if attr.has_one.is_some() {
                    relation_type = quote! { has_one };
                    attr.has_one
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'has_one'")
                        })
                } else if attr.has_many.is_some() {
                    relation_type = quote! { has_many };
                    attr.has_many
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'has_many'")
                        })
                } else {
                    Err(syn::Error::new_spanned(
                        variant,
                        "Missing one of 'has_one', 'has_many' or 'belongs_to'",
                    ))
                }??;

                let mut result = quote!(
                    Self::#variant_ident => #entity_ident::#relation_type(#related_to)
                );

                if attr.from.is_some() {
                    let from =
                        attr.from
                            .as_ref()
                            .map(Self::parse_lit_string)
                            .ok_or_else(|| {
                                syn::Error::new_spanned(variant, "Missing value for 'from'")
                            })??;
                    result = quote! { #result.from(#from) };
                } else if attr.belongs_to.is_some() {
                    return Err(syn::Error::new_spanned(variant, "Missing attribute 'from'"));
                }

                if attr.to.is_some() {
                    let to = attr
                        .to
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'to'")
                        })??;
                    result = quote! { #result.to(#to) };
                } else if attr.belongs_to.is_some() {
                    return Err(syn::Error::new_spanned(variant, "Missing attribute 'to'"));
                }

                if attr.on_update.is_some() {
                    let on_update = attr
                        .on_update
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'on_update'")
                        })??;
                    result = quote! { #result.on_update(sea_orm::prelude::ForeignKeyAction::#on_update) };
                }

                if attr.on_delete.is_some() {
                    let on_delete = attr
                        .on_delete
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'on_delete'")
                        })??;
                    result = quote! { #result.on_delete(sea_orm::prelude::ForeignKeyAction::#on_delete) };
                }

                if attr.on_condition.is_some() {
                    let on_condition = attr
                        .on_condition
                        .as_ref()
                        .map(Self::parse_lit_string)
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'on_condition'")
                        })??;
                    result = quote! { #result.on_condition(|_, _| sea_orm::sea_query::IntoCondition::into_condition(#on_condition)) };
                }

                if attr.fk_name.is_some() {
                    let fk_name = attr
                        .fk_name
                        .as_ref()
                        .map(|lit| {
                            match lit {
                                syn::Lit::Str(lit_str) => Ok(lit_str.value()),
                                _ => Err(syn::Error::new_spanned(lit, "attribute must be a string")),
                            }
                        })
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'fk_name'")
                        })??;
                    result = quote! { #result.fk_name(#fk_name) };
                }

                if attr.condition_type.is_some() {
                    let condition_type = attr
                        .condition_type
                        .as_ref()
                        .map(|lit| {
                            match lit {
                                syn::Lit::Str(lit_str) => {
                                    match lit_str.value().to_ascii_lowercase().as_str() {
                                        "all" => Ok(quote!( sea_orm::sea_query::ConditionType::All )),
                                        "any" => Ok(quote!( sea_orm::sea_query::ConditionType::Any )),
                                        _ => Err(syn::Error::new_spanned(lit, "Condition type must be one of `all` or `any`")),
                                    }
                                },
                                _ => Err(syn::Error::new_spanned(lit, "attribute must be a string")),
                            }
                        })
                        .ok_or_else(|| {
                            syn::Error::new_spanned(variant, "Missing value for 'condition_type'")
                        })??;
                    result = quote! { #result.condition_type(#condition_type) };
                }

                result = quote! { #result.into() };

                Result::<_, syn::Error>::Ok(result)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(quote!(
            #[automatically_derived]
            impl sea_orm::entity::RelationTrait for #ident {
                fn def(&self) -> sea_orm::entity::RelationDef {
                    match self {
                        #( #variant_relation_defs, )*
                        _ => panic!(#no_relation_def_msg)
                    }
                }
            }
        ))
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

/// Method to derive a Relation
pub fn expand_derive_relation(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DeriveRelation::new(input) {
        Ok(model) => model.expand(),
        Err(Error::InputNotEnum) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive DeriveRelation on enums");
        }),
        Err(Error::Syn(err)) => Err(err),
    }
}
