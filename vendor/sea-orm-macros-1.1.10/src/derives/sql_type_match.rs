use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{LitStr, Type};

pub fn col_type_match(
    col_type: Option<TokenStream>,
    field_type: &str,
    field_span: Span,
) -> TokenStream {
    match col_type {
        Some(t) => quote! { sea_orm::prelude::ColumnType::#t },
        None => {
            let col_type = match field_type {
                "char" => quote! { Char(None) },
                "String" | "&str" => quote! { string(None) },
                "i8" => quote! { TinyInteger },
                "u8" => quote! { TinyUnsigned },
                "i16" => quote! { SmallInteger },
                "u16" => quote! { SmallUnsigned },
                "i32" => quote! { Integer },
                "u32" => quote! { Unsigned },
                "i64" => quote! { BigInteger },
                "u64" => quote! { BigUnsigned },
                "f32" => quote! { Float },
                "f64" => quote! { Double },
                "bool" => quote! { Boolean },
                "Date" | "NaiveDate" => quote! { Date },
                "Time" | "NaiveTime" => quote! { Time },
                "DateTime" | "NaiveDateTime" => {
                    quote! { DateTime }
                }
                "DateTimeUtc" | "DateTimeLocal" | "DateTimeWithTimeZone" => {
                    quote! { TimestampWithTimeZone }
                }
                "Uuid" => quote! { Uuid },
                "Json" => quote! { Json },
                "Decimal" => quote! { Decimal(None) },
                "Vec<u8>" => {
                    quote! { VarBinary(sea_orm::sea_query::StringLen::None) }
                }
                _ => {
                    // Assumed it's ActiveEnum if none of the above type matches
                    quote! {}
                }
            };
            if col_type.is_empty() {
                let ty: Type = LitStr::new(field_type, field_span)
                    .parse()
                    .expect("field type error");
                let def = quote_spanned! { field_span =>
                    std::convert::Into::<sea_orm::sea_query::ColumnType>::into(
                        <#ty as sea_orm::sea_query::ValueType>::column_type()
                    )
                };
                quote! { #def }
            } else {
                quote! { sea_orm::prelude::ColumnType::#col_type }
            }
        }
    }
}

pub fn arr_type_match(
    arr_type: Option<TokenStream>,
    field_type: &str,
    field_span: Span,
) -> TokenStream {
    match arr_type {
        Some(t) => quote! { sea_orm::sea_query::ArrayType::#t },
        None => {
            let arr_type = match field_type {
                "char" => quote! { Char },
                "String" | "&str" => quote! { String },
                "i8" => quote! { TinyInt },
                "u8" => quote! { TinyUnsigned },
                "i16" => quote! { SmallInt },
                "u16" => quote! { SmallUnsigned },
                "i32" => quote! { Int },
                "u32" => quote! { Unsigned },
                "i64" => quote! { BigInt },
                "u64" => quote! { BigUnsigned },
                "f32" => quote! { Float },
                "f64" => quote! { Double },
                "bool" => quote! { Bool },
                "Date" | "NaiveDate" => quote! { ChronoDate },
                "Time" | "NaiveTime" => quote! { ChronoTime },
                "DateTime" | "NaiveDateTime" => {
                    quote! { ChronoDateTime }
                }
                "DateTimeUtc" | "DateTimeLocal" | "DateTimeWithTimeZone" => {
                    quote! { ChronoDateTimeWithTimeZone }
                }
                "Uuid" => quote! { Uuid },
                "Json" => quote! { Json },
                "Decimal" => quote! { Decimal },
                _ => {
                    // Assumed it's ActiveEnum if none of the above type matches
                    quote! {}
                }
            };
            if arr_type.is_empty() {
                let ty: Type = LitStr::new(field_type, field_span)
                    .parse()
                    .expect("field type error");
                let def = quote_spanned! { field_span =>
                    std::convert::Into::<sea_orm::sea_query::ArrayType>::into(
                        <#ty as sea_orm::sea_query::ValueType>::array_type()
                    )
                };
                quote! { #def }
            } else {
                quote! { sea_orm::sea_query::ArrayType::#arr_type }
            }
        }
    }
}
