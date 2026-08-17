use std::iter::FromIterator;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::attributes::derive_attr;

struct DeriveEntity {
    column_ident: syn::Ident,
    ident: syn::Ident,
    model_ident: syn::Ident,
    active_model_ident: syn::Ident,
    primary_key_ident: syn::Ident,
    relation_ident: syn::Ident,
    schema_name: Option<syn::Lit>,
    table_name: Option<syn::Lit>,
}

impl DeriveEntity {
    fn new(input: syn::DeriveInput) -> Result<Self, syn::Error> {
        let sea_attr = derive_attr::SeaOrm::try_from_attributes(&input.attrs)?.unwrap_or_default();

        let ident = input.ident;
        let column_ident = sea_attr.column.unwrap_or_else(|| format_ident!("Column"));
        let model_ident = sea_attr.model.unwrap_or_else(|| format_ident!("Model"));
        let active_model_ident = sea_attr
            .active_model
            .unwrap_or_else(|| format_ident!("ActiveModel"));
        let primary_key_ident = sea_attr
            .primary_key
            .unwrap_or_else(|| format_ident!("PrimaryKey"));
        let relation_ident = sea_attr
            .relation
            .unwrap_or_else(|| format_ident!("Relation"));

        let table_name = sea_attr.table_name;
        let schema_name = sea_attr.schema_name;

        Ok(DeriveEntity {
            column_ident,
            ident,
            model_ident,
            active_model_ident,
            primary_key_ident,
            relation_ident,
            schema_name,
            table_name,
        })
    }

    fn expand(&self) -> TokenStream {
        let expanded_impl_entity_name = self.impl_entity_name();
        let expanded_impl_entity_trait = self.impl_entity_trait();
        let expanded_impl_iden = self.impl_iden();
        let expanded_impl_iden_static = self.impl_iden_static();

        TokenStream::from_iter([
            expanded_impl_entity_name,
            expanded_impl_entity_trait,
            expanded_impl_iden,
            expanded_impl_iden_static,
        ])
    }

    fn impl_entity_name(&self) -> TokenStream {
        let ident = &self.ident;
        let table_name = match &self.table_name {
            Some(table_name) => table_name,
            None => return TokenStream::new(), // No table name, do not derive EntityName
        };
        let expanded_schema_name = self
            .schema_name
            .as_ref()
            .map(|schema| quote!(Some(#schema)))
            .unwrap_or_else(|| quote!(None));

        quote!(
            #[automatically_derived]
            impl sea_orm::entity::EntityName for #ident {
                fn schema_name(&self) -> Option<&str> {
                    #expanded_schema_name
                }

                fn table_name(&self) -> &str {
                    #table_name
                }
            }
        )
    }

    fn impl_entity_trait(&self) -> TokenStream {
        let Self {
            ident,
            model_ident,
            active_model_ident,
            column_ident,
            primary_key_ident,
            relation_ident,
            ..
        } = self;

        quote!(
            #[automatically_derived]
            impl sea_orm::entity::EntityTrait for #ident {
                type Model = #model_ident;

                type ActiveModel = #active_model_ident;

                type Column = #column_ident;

                type PrimaryKey = #primary_key_ident;

                type Relation = #relation_ident;
            }
        )
    }

    fn impl_iden(&self) -> TokenStream {
        let ident = &self.ident;

        quote!(
            #[automatically_derived]
            impl sea_orm::Iden for #ident {
                fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                    write!(s, "{}", sea_orm::IdenStatic::as_str(self)).unwrap();
                }
            }
        )
    }

    fn impl_iden_static(&self) -> TokenStream {
        let ident = &self.ident;

        quote!(
            #[automatically_derived]
            impl sea_orm::IdenStatic for #ident {
                fn as_str(&self) -> &str {
                    <Self as sea_orm::EntityName>::table_name(self)
                }
            }
        )
    }
}

pub fn expand_derive_entity(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    Ok(DeriveEntity::new(input)?.expand())
}
