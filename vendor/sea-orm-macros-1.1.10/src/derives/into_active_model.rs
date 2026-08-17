use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{punctuated::Punctuated, token::Comma, Meta, PathArguments, PathSegment};

use super::util::GetMeta;

enum Error {
    InputNotStruct,
    Syn(syn::Error),
}

pub(super) struct DeriveIntoActiveModel {
    pub ident: syn::Ident,
    pub active_model: Option<syn::Type>,
    pub fields: Vec<syn::Ident>,
}

impl DeriveIntoActiveModel {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        let mut active_model = None;

        for attr in input.attrs.iter() {
            if !attr.path().is_ident("sea_orm") {
                continue;
            }

            if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                for meta in list {
                    if let Some(s) = meta.get_as_kv("active_model") {
                        active_model = Some(syn::parse_str::<syn::Type>(&s).map_err(Error::Syn)?);
                    }
                }
            }
        }

        let field_idents = fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().clone())
            .collect();

        Ok(Self {
            ident: input.ident,
            active_model,
            fields: field_idents,
        })
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        let expanded_impl_into_active_model = self.impl_into_active_model();

        Ok(expanded_impl_into_active_model)
    }

    pub(super) fn impl_into_active_model(&self) -> TokenStream {
        let Self {
            ident,
            active_model,
            fields,
        } = self;

        let mut active_model_ident = active_model
            .clone()
            .unwrap_or_else(|| syn::parse_str::<syn::Type>("ActiveModel").unwrap());

        let type_alias_definition = if is_qualified_type(&active_model_ident) {
            let type_alias = format_ident!("ActiveModelFor{ident}");
            let type_def = quote!( type #type_alias = #active_model_ident; );
            active_model_ident = syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: [PathSegment {
                        ident: type_alias,
                        arguments: PathArguments::None,
                    }]
                    .into_iter()
                    .collect(),
                },
            });
            type_def
        } else {
            quote!()
        };

        let expanded_fields = fields.iter().map(|field_ident| {
            quote!(
                sea_orm::IntoActiveValue::<_>::into_active_value(self.#field_ident).into()
            )
        });

        quote!(
            #type_alias_definition

            #[automatically_derived]
            impl sea_orm::IntoActiveModel<#active_model_ident> for #ident {
                fn into_active_model(self) -> #active_model_ident {
                    #active_model_ident {
                        #( #fields: #expanded_fields, )*
                        ..::std::default::Default::default()
                    }
                }
            }
        )
    }
}

/// Method to derive the ActiveModel from the [ActiveModelTrait](sea_orm::ActiveModelTrait)
pub fn expand_into_active_model(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DeriveIntoActiveModel::new(input) {
        Ok(model) => model.expand(),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive IntoActiveModel on structs");
        }),
        Err(Error::Syn(err)) => Err(err),
    }
}

fn is_qualified_type(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(syn::TypePath { qself: Some(_), .. }))
}
