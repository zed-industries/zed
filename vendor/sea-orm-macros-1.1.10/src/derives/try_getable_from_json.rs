use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn expand_derive_from_json_query_result(ident: Ident) -> syn::Result<TokenStream> {
    let impl_not_u8 = if cfg!(feature = "postgres-array") {
        quote!(
            #[automatically_derived]
            impl sea_orm::sea_query::value::with_array::NotU8 for #ident {}
        )
    } else {
        quote!()
    };

    Ok(quote!(
        #[automatically_derived]
        impl sea_orm::TryGetableFromJson for #ident {}

        #[automatically_derived]
        impl std::convert::From<#ident> for sea_orm::Value {
            fn from(source: #ident) -> Self {
                sea_orm::Value::Json(serde_json::to_value(&source).ok().map(|s| std::boxed::Box::new(s)))
            }
        }

        #[automatically_derived]
        impl sea_orm::sea_query::ValueType for #ident {
            fn try_from(v: sea_orm::Value) -> Result<Self, sea_orm::sea_query::ValueTypeErr> {
                match v {
                    sea_orm::Value::Json(Some(json)) => Ok(
                        serde_json::from_value(*json).map_err(|_| sea_orm::sea_query::ValueTypeErr)?,
                    ),
                    _ => Err(sea_orm::sea_query::ValueTypeErr),
                }
            }

            fn type_name() -> String {
                stringify!(#ident).to_owned()
            }

            fn array_type() -> sea_orm::sea_query::ArrayType {
                sea_orm::sea_query::ArrayType::Json
            }

            fn column_type() -> sea_orm::sea_query::ColumnType {
                sea_orm::sea_query::ColumnType::Json
            }
        }

        #[automatically_derived]
        impl sea_orm::sea_query::Nullable for #ident {
            fn null() -> sea_orm::Value {
                sea_orm::Value::Json(None)
            }
        }

        #impl_not_u8
    ))
}
