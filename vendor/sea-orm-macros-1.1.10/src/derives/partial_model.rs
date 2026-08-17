use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    ext::IdentExt, punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, Meta, Type,
};

use super::from_query_result::{
    DeriveFromQueryResult, FromQueryResultItem, ItemType as FqrItemType,
};
use super::into_active_model::DeriveIntoActiveModel;
use super::util::GetMeta;

#[derive(Debug)]
enum Error {
    InputNotStruct,
    EntityNotSpecified,
    NotSupportGeneric(Span),
    OverlappingAttributes(Span),
    Syn(syn::Error),
}

#[derive(Debug, PartialEq, Eq)]
enum ColumnAs {
    /// alias from a column in model
    Col {
        col: Option<syn::Ident>,
        field: syn::Ident,
    },
    /// from an expr
    Expr {
        expr: syn::Expr,
        field: syn::Ident,
    },
    /// nesting another struct
    Nested {
        typ: Type,
        field: syn::Ident,
    },
    Skip(syn::Ident),
}

struct DerivePartialModel {
    entity: Option<syn::Type>,
    active_model: Option<syn::Type>,
    alias: Option<String>,
    ident: syn::Ident,
    fields: Vec<ColumnAs>,
    from_query_result: bool,
    into_active_model: bool,
}

impl DerivePartialModel {
    fn new(input: syn::DeriveInput) -> Result<Self, Error> {
        if !input.generics.params.is_empty() {
            return Err(Error::NotSupportGeneric(input.generics.params.span()));
        }

        let fields = match input.data {
            syn::Data::Struct(syn::DataStruct {
                fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => return Err(Error::InputNotStruct),
        };

        let mut entity = None;
        let mut entity_string = String::new();
        let mut active_model = None;
        let mut alias = None;
        let mut from_query_result = false;
        let mut into_active_model = false;

        for attr in input.attrs.iter() {
            if !attr.path().is_ident("sea_orm") {
                continue;
            }

            if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                for meta in list {
                    if let Some(s) = meta.get_as_kv("entity") {
                        entity = Some(syn::parse_str::<syn::Type>(&s).map_err(Error::Syn)?);
                        entity_string = s;
                    } else if let Some(s) = meta.get_as_kv("alias") {
                        alias = Some(s);
                    } else if meta.exists("from_query_result") {
                        from_query_result = true;
                    } else if meta.exists("into_active_model") {
                        into_active_model = true;
                    }
                }
            }
        }

        if into_active_model {
            active_model = Some(
                syn::parse_str::<syn::Type>(&format!(
                    "<{entity_string} as EntityTrait>::ActiveModel"
                ))
                .map_err(Error::Syn)?,
            );
        }

        let mut column_as_list = Vec::with_capacity(fields.len());

        for field in fields {
            let field_span = field.span();

            let mut from_col = None;
            let mut from_expr = None;
            let mut nested = false;
            let mut skip = false;

            for attr in field.attrs.iter() {
                if !attr.path().is_ident("sea_orm") {
                    continue;
                }

                if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)
                {
                    for meta in list.iter() {
                        if meta.exists("skip") {
                            skip = true;
                        } else if meta.exists("nested") {
                            nested = true;
                        } else if let Some(s) = meta.get_as_kv("from_col") {
                            from_col = Some(format_ident!("{}", s.to_upper_camel_case()));
                        } else if let Some(s) = meta.get_as_kv("from_expr") {
                            from_expr = Some(syn::parse_str::<Expr>(&s).map_err(Error::Syn)?);
                        }
                    }
                }
            }

            let field_name = field.ident.unwrap();

            let col_as = match (from_col, from_expr, nested) {
                (Some(col), None, false) => {
                    if entity.is_none() {
                        return Err(Error::EntityNotSpecified);
                    }

                    ColumnAs::Col {
                        col: Some(col),
                        field: field_name,
                    }
                }
                (None, Some(expr), false) => ColumnAs::Expr {
                    expr,
                    field: field_name,
                },
                (None, None, true) => ColumnAs::Nested {
                    typ: field.ty,
                    field: field_name,
                },
                (None, None, false) => {
                    if entity.is_none() {
                        return Err(Error::EntityNotSpecified);
                    }
                    if skip {
                        ColumnAs::Skip(field_name)
                    } else {
                        ColumnAs::Col {
                            col: None,
                            field: field_name,
                        }
                    }
                }
                (_, _, _) => return Err(Error::OverlappingAttributes(field_span)),
            };
            column_as_list.push(col_as);
        }

        Ok(Self {
            entity,
            active_model,
            alias,
            ident: input.ident,
            fields: column_as_list,
            from_query_result,
            into_active_model,
        })
    }

    fn expand(&self) -> syn::Result<TokenStream> {
        let impl_partial_model = self.impl_partial_model();

        let impl_from_query_result = if self.from_query_result {
            DeriveFromQueryResult {
                ident: self.ident.clone(),
                generics: Default::default(),
                fields: self
                    .fields
                    .iter()
                    .map(|col_as| FromQueryResultItem {
                        typ: match col_as {
                            ColumnAs::Nested { .. } => FqrItemType::Nested,
                            ColumnAs::Skip(_) => FqrItemType::Skip,
                            _ => FqrItemType::Flat,
                        },
                        ident: match col_as {
                            ColumnAs::Col { field, .. } => field,
                            ColumnAs::Expr { field, .. } => field,
                            ColumnAs::Nested { field, .. } => field,
                            ColumnAs::Skip(field) => field,
                        }
                        .to_owned(),
                        alias: None,
                    })
                    .collect(),
            }
            .impl_from_query_result(true)
        } else {
            quote!()
        };

        let impl_into_active_model = if self.into_active_model {
            DeriveIntoActiveModel {
                ident: self.ident.clone(),
                active_model: self.active_model.clone(),
                fields: self
                    .fields
                    .iter()
                    .filter_map(|col_as| {
                        match col_as {
                            ColumnAs::Col { field, .. } => Some(field),
                            ColumnAs::Expr { field, .. } => Some(field),
                            ColumnAs::Nested { .. } => None,
                            ColumnAs::Skip(_) => None,
                        }
                        .cloned()
                    })
                    .collect(),
            }
            .impl_into_active_model()
        } else {
            quote!()
        };

        Ok(quote! {
            #impl_partial_model
            #impl_from_query_result
            #impl_into_active_model
        })
    }

    fn impl_partial_model(&self) -> TokenStream {
        let select_ident = format_ident!("select");
        let DerivePartialModel {
            entity,
            alias,
            ident,
            fields,
            ..
        } = self;
        let select_col_code_gen = fields.iter().map(|col_as| match col_as {
            ColumnAs::Col { col, field } => {
                let field = field.unraw().to_string();
                let entity = entity.as_ref().unwrap();
                let col_name = if let Some(col) = col {
                    col
                } else {
                    &format_ident!("{}", field.to_upper_camel_case())
                };
                let col_value = if let Some(alias) = alias {
                    quote!(sea_orm::sea_query::Expr::col((sea_orm::sea_query::Alias::new(#alias), <#entity as sea_orm::EntityTrait>::Column:: #col_name)))
                } else {
                    quote!( <#entity as sea_orm::EntityTrait>::Column:: #col_name)
                };
                quote!(let #select_ident =
                    if let Some(prefix) = pre {
                        let ident = format!("{prefix}{}", #field);
                        sea_orm::SelectColumns::select_column_as(#select_ident, #col_value, ident)
                    } else {
                        sea_orm::SelectColumns::select_column_as(#select_ident, #col_value, #field)
                    };
                )
            }
            ColumnAs::Expr { expr, field } => {
                let field = field.unraw().to_string();
                quote!(let #select_ident =
                    if let Some(prefix) = pre {
                        let ident = format!("{prefix}{}", #field);
                        sea_orm::SelectColumns::select_column_as(#select_ident, #expr, ident)
                    } else {
                        sea_orm::SelectColumns::select_column_as(#select_ident, #expr, #field)
                    };
                )
            }
            ColumnAs::Nested { typ, field } => {
                let field = field.unraw().to_string();
                quote!(let #select_ident =
                    <#typ as sea_orm::PartialModelTrait>::select_cols_nested(#select_ident,
                        Some(&if let Some(prefix) = pre {
                                format!("{prefix}{}_", #field)
                            } else {
                                format!("{}_", #field)
                            }
                        ));
                )
            }
            ColumnAs::Skip(_) => quote!(),
        });

        quote! {
            #[automatically_derived]
            impl sea_orm::PartialModelTrait for #ident {
                fn select_cols<S: sea_orm::SelectColumns>(#select_ident: S) -> S {
                    Self::select_cols_nested(#select_ident, None)
                }

                fn select_cols_nested<S: sea_orm::SelectColumns>(#select_ident: S, pre: Option<&str>) -> S {
                    #(#select_col_code_gen)*
                    #select_ident
                }
            }
        }
    }
}

pub fn expand_derive_partial_model(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let ident_span = input.ident.span();

    match DerivePartialModel::new(input) {
        Ok(partial_model) => partial_model.expand(),
        Err(Error::NotSupportGeneric(span)) => Ok(quote_spanned! {
            span => compile_error!("you can only derive `DerivePartialModel` on concrete struct");
        }),
        Err(Error::OverlappingAttributes(span)) => Ok(quote_spanned! {
            span => compile_error!("you can only use one of `from_col`, `from_expr`, `nested`");
        }),
        Err(Error::EntityNotSpecified) => Ok(quote_spanned! {
            ident_span => compile_error!("you need specific which entity you are using")
        }),
        Err(Error::InputNotStruct) => Ok(quote_spanned! {
            ident_span => compile_error!("you can only derive `DerivePartialModel` on named struct");
        }),
        Err(Error::Syn(err)) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use quote::format_ident;
    use syn::{parse_str, DeriveInput, Type};

    use crate::derives::partial_model::ColumnAs;

    use super::DerivePartialModel;

    type StdResult<T> = Result<T, Box<dyn std::error::Error>>;

    const CODE_SNIPPET_1: &str = r#"
        #[sea_orm(entity = "Entity")]
        struct PartialModel {
            default_field: i32,
            #[sea_orm(from_col = "bar")]
            alias_field: i32,
            #[sea_orm(from_expr = "Expr::val(1).add(1)")]
            expr_field : i32
        }
        "#;

    #[test]
    fn test_load_macro_input_1() -> StdResult<()> {
        let input = parse_str::<DeriveInput>(CODE_SNIPPET_1)?;

        let middle = DerivePartialModel::new(input).unwrap();
        assert_eq!(middle.entity, Some(parse_str::<Type>("Entity").unwrap()));
        assert_eq!(middle.ident, format_ident!("PartialModel"));
        assert_eq!(middle.fields.len(), 3);
        assert_eq!(
            middle.fields[0],
            ColumnAs::Col {
                col: None,
                field: format_ident!("default_field")
            }
        );
        assert_eq!(
            middle.fields[1],
            ColumnAs::Col {
                col: Some(format_ident!("Bar")),
                field: format_ident!("alias_field"),
            },
        );
        assert_eq!(
            middle.fields[2],
            ColumnAs::Expr {
                expr: syn::parse_str("Expr::val(1).add(1)").unwrap(),
                field: format_ident!("expr_field"),
            }
        );
        assert_eq!(middle.from_query_result, false);

        Ok(())
    }

    const CODE_SNIPPET_2: &str = r#"
        #[sea_orm(entity = "MyEntity", from_query_result)]
        struct PartialModel {
            default_field: i32,
        }
        "#;

    #[test]
    fn test_load_macro_input_2() -> StdResult<()> {
        let input = parse_str::<DeriveInput>(CODE_SNIPPET_2)?;

        let middle = DerivePartialModel::new(input).unwrap();
        assert_eq!(middle.entity, Some(parse_str::<Type>("MyEntity").unwrap()));
        assert_eq!(middle.ident, format_ident!("PartialModel"));
        assert_eq!(middle.fields.len(), 1);
        assert_eq!(
            middle.fields[0],
            ColumnAs::Col {
                col: None,
                field: format_ident!("default_field")
            }
        );
        assert_eq!(middle.from_query_result, true);

        Ok(())
    }
}
