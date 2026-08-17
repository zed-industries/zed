use crate::datastructure::{DataStructure, DataVariant, Field, StructKind};

use proc_macro2::{Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned, quote_spanned as quote_s, ToTokens, TokenStreamExt};

use syn::{DeriveInput, Ident};

mod attribute_parsing;
mod syntax;
mod type_detection;

use self::attribute_parsing::HowToFmt;

pub(crate) fn derive_constdebug_impl(input: DeriveInput) -> Result<TokenStream2, crate::Error> {
    let ds = &DataStructure::new(&input);
    let config = attribute_parsing::parse_attrs_for_derive(ds)?;
    let cratep = match &config.crate_path {
        Some(p) => p.to_token_stream(),
        None => quote!(::const_format),
    };

    let vis = ds.vis;

    let name = ds.name;

    let mut impl_headers;

    if config.impls.is_empty() {
        let impl_params = ds.generics.params.iter();
        let (_, tygen, _) = ds.generics.split_for_impl();
        let where_clause = get_where_clause_tokens(&ds.generics.where_clause);

        impl_headers = quote!(
            impl[#( #impl_params ,)*] #name #tygen
            #where_clause;
        );
    } else {
        impl_headers = TokenStream2::new();

        for imp in config.impls.iter() {
            let params = imp.generics.params.iter();
            let self_ty = &imp.self_ty;
            let where_clause = get_where_clause_tokens(&imp.generics.where_clause);

            impl_headers.append_all(quote!(
                impl[#(#params)*] #self_ty
                #where_clause;
            ));
        }
    };

    let enum_prefix = match ds.data_variant {
        DataVariant::Enum => quote!(#name::),
        DataVariant::Struct => TokenStream2::new(),
        DataVariant::Union => panic!("Cannot derive ConstDebug on unions"),
    };

    let variant_branches = ds.variants.iter().map(|variant| {
        let vname = variant.name;

        let debug_method = match variant.kind {
            StructKind::Braced => Ident::new("debug_struct", Span::call_site()),
            StructKind::Tupled => Ident::new("debug_tuple", Span::call_site()),
        };

        let patt = variant
            .fields
            .iter()
            .filter_map(|f| -> Option<TokenStream2> {
                if let HowToFmt::Ignore = config.field_map[f].how_to_fmt {
                    return None;
                }

                let pat = &f.ident;
                let variable = f.pattern_ident();

                Some(quote!(#pat : #variable,))
            });

        let fmt_call = variant
            .fields
            .iter()
            .filter_map(|f| -> Option<TokenStream2> {
                let how_to_fmt = &config.field_map[f].how_to_fmt;
                if let HowToFmt::Ignore = how_to_fmt {
                    return None;
                }

                let fspan = f.pattern_ident().span();

                let field_name_str = match variant.kind {
                    StructKind::Braced => Some(f.ident.to_string()),
                    StructKind::Tupled => None,
                }
                .into_iter();

                let mut field_ts = quote_spanned!(fspan=>
                    let mut field_formatter = formatter.field(#(#field_name_str)*);
                );

                field_ts.append_all(match &how_to_fmt {
                    HowToFmt::Regular => coerce_and_fmt(&cratep, f),
                    HowToFmt::Ignore => unreachable!(),
                    HowToFmt::Slice => fmt_slice(&cratep, f),
                    HowToFmt::Option_ => fmt_option(&cratep, f),
                    HowToFmt::Newtype(newtype) => fmt_newtype(&cratep, newtype, f),
                    HowToFmt::With(with) => call_with_function(&cratep, f, with),
                    HowToFmt::WithMacro(with) => call_with_macro(&cratep, f, with),
                    HowToFmt::WithWrapper(with) => call_with_wrapper(&cratep, f, with),
                });

                Some(field_ts)
            });

        quote!(
            #enum_prefix #vname { #(#patt)* .. } => {
                let mut formatter = formatter.#debug_method(stringify!(#vname));
                #(#fmt_call)*
                formatter.finish()
            }
        )
    });

    let ret = quote!(
        #cratep::impl_fmt!{
            #impl_headers

            #vis const fn const_debug_fmt(
                &self,
                formatter: &mut #cratep::pmr::Formatter<'_>,
            ) -> #cratep::pmr::Result<(), #cratep::pmr::Error> {
                match self {
                    #(
                        #variant_branches
                    )*
                }
            }
        }
    );

    if config.debug_print {
        panic!("\n\n\n{}\n\n\n", ret);
    }
    Ok(ret)
}

// Copying the definitino of the `const_format::coerce_to_fn` macro here
// because the compiler points inside the coerce_to_fn macro otherwise
fn coerce_and_fmt(cratep: &TokenStream2, field: &Field<'_>) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    quote_spanned!(fspan=>
        let mut marker = #cratep::pmr::IsAFormatMarker::NEW;
        if false {
            marker = marker.infer_type(#var);
        }
        #cratep::try_!(
            marker.coerce(marker.unreference(#var))
                .const_debug_fmt(field_formatter)
        );
    )
}

fn fmt_slice(cratep: &TokenStream2, field: &Field<'_>) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    let call = call_debug_fmt(
        cratep,
        quote_s!(fspan=> &#var[n]),
        quote_s!(fspan=> slice_fmt.entry()),
        fspan,
    );

    quote_spanned!(fspan=>{
        let mut slice_fmt = field_formatter.debug_list();
        let mut n = 0;
        let len = #var.len();
        while n != len {
            #call
            n += 1;
        }
        #cratep::try_!(slice_fmt.finish());
    })
}

fn fmt_option(cratep: &TokenStream2, field: &Field<'_>) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    let call = call_debug_fmt(
        cratep,
        quote_s!(fspan=>val),
        quote_s!(fspan=>f.field()),
        fspan,
    );

    quote_spanned!(fspan=>
        #cratep::try_!(match #var {
            #cratep::pmr::Some(val) => {
                let mut f = field_formatter.debug_tuple("Some");
                #call
                f.finish()
            }
            #cratep::pmr::None => field_formatter.write_str("None"),
        });
    )
}

fn fmt_newtype(cratep: &TokenStream2, newtype: &syn::Ident, field: &Field<'_>) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();
    let ty_str = newtype.to_token_stream().to_string();

    let call = call_debug_fmt(
        cratep,
        quote_s!(fspan=> &#var.0 ),
        quote_s!(fspan=> f.field() ),
        fspan,
    );

    quote_spanned!(fspan=>{
        #cratep::try_!({
            let mut f = field_formatter.debug_tuple(#ty_str);
            #call
            f.finish()
        });
    })
}

fn call_with_function(cratep: &TokenStream2, field: &Field<'_>, func: &syn::Path) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    quote_spanned!(fspan=> #cratep::try_!(#func(#var, field_formatter)); )
}

fn call_with_macro(cratep: &TokenStream2, field: &Field<'_>, macr: &syn::Path) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    quote_spanned!(fspan=> #cratep::try_!(#macr!(#var, field_formatter)); )
}

fn call_with_wrapper(
    cratep: &TokenStream2,
    field: &Field<'_>,
    newtype: &syn::Path,
) -> TokenStream2 {
    let var = field.pattern_ident();
    let fspan = var.span();

    quote_spanned!(fspan=>
        #cratep::try_!(#newtype(#var).const_debug_fmt(field_formatter));
    )
}

// Helper of the other `call_` functions
fn call_debug_fmt(
    cratep: &TokenStream2,
    field: impl ToTokens,
    formatter: impl ToTokens,
    span: Span,
) -> TokenStream2 {
    quote_spanned!(span=>{
        // Importing it like this because the error span is wrong otherwise
        use #cratep::pmr::IsAFormatMarker as __IsAFormatMarker;

        let mut marker = __IsAFormatMarker::NEW;
        if false {
            marker = marker.infer_type(#field);
        }
        #cratep::try_!(
            marker.coerce(marker.unreference(#field)).const_debug_fmt(#formatter)
        );
    })
}

fn get_where_clause_tokens(where_clause: &Option<syn::WhereClause>) -> TokenStream2 {
    match where_clause {
        Some(x) => {
            let preds = x.predicates.iter();
            quote!(where[ #(#preds,)* ])
        }
        None => TokenStream2::new(),
    }
}
