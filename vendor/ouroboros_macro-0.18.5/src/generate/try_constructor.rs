use crate::{
    info_structures::{ArgType, BuilderType, FieldType, Options, StructInfo},
    utils::to_class_case,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

pub fn create_try_builder_and_constructor(
    info: &StructInfo,
    options: Options,
    builder_type: BuilderType,
) -> Result<(Ident, TokenStream, TokenStream), Error> {
    let struct_name = info.ident.clone();
    let generic_args = info.generic_arguments();

    let visibility = if options.do_pub_extras {
        info.vis.clone()
    } else {
        syn::parse_quote! { pub(super) }
    };
    let mut head_recover_code = Vec::new();
    for field in &info.fields {
        if !field.self_referencing {
            let field_name = &field.name;
            head_recover_code.push(quote! { #field_name });
        }
    }
    for (_ty, ident) in info.generic_consumers() {
        head_recover_code.push(quote! { #ident: ::core::marker::PhantomData });
    }
    let mut current_head_index = 0;

    let builder_struct_name = match builder_type {
        BuilderType::AsyncSend => format_ident!("{}AsyncSendTryBuilder", info.ident),
        BuilderType::Async => format_ident!("{}AsyncTryBuilder", info.ident),
        BuilderType::Sync => format_ident!("{}TryBuilder", info.ident),
    };
    let documentation = format!(
        concat!(
            "(See also [`{0}::try_build()`]({0}::try_build).) Like [`new`](Self::new), but ",
            "builders for [self-referencing fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) ",
            "can return results. If any of them fail, `Err` is returned. If all of them ",
            "succeed, `Ok` is returned. The arguments are as follows:\n\n",
            "| Argument | Suggested Use |\n| --- | --- |\n",
        ),
        builder_struct_name.to_string()
    );
    let or_recover_documentation = format!(
        concat!(
            "(See also [`{0}::try_build_or_recover()`]({0}::try_build_or_recover).) Like ",
            "[`try_new`](Self::try_new), but all ",
            "[head fields](https://docs.rs/ouroboros/latest/ouroboros/attr.self_referencing.html#definitions) ",
            "are returned in the case of an error. The arguments are as follows:\n\n",
            "| Argument | Suggested Use |\n| --- | --- |\n",
        ),
        builder_struct_name.to_string()
    );
    let builder_documentation = concat!(
        "A more verbose but stable way to construct self-referencing structs. It is ",
        "comparable to using `StructName { field1: value1, field2: value2 }` rather than ",
        "`StructName::new(value1, value2)`. This has the dual benefit of making your code ",
        "both easier to refactor and more readable. Call [`try_build()`](Self::try_build) or ",
        "[`try_build_or_recover()`](Self::try_build_or_recover) to ",
        "construct the actual struct. The fields of this struct should be used as follows:\n\n",
        "| Field | Suggested Use |\n| --- | --- |\n",
    )
    .to_owned();
    let build_fn_documentation = format!(
        concat!(
            "Calls [`{0}::try_new()`]({0}::try_new) using the provided values. This is ",
            "preferable over calling `try_new()` directly for the reasons listed above. "
        ),
        info.ident.to_string()
    );
    let build_or_recover_fn_documentation = format!(
        concat!(
            "Calls [`{0}::try_new_or_recover()`]({0}::try_new_or_recover) using the provided ",
            "values. This is preferable over calling `try_new_or_recover()` directly for the ",
            "reasons listed above. "
        ),
        info.ident.to_string()
    );
    let mut doc_table = "".to_owned();
    let mut or_recover_code: Vec<TokenStream> = Vec::new();
    let mut params: Vec<TokenStream> = Vec::new();
    let mut builder_struct_generic_producers: Vec<_> = info
        .generic_params()
        .iter()
        .map(|param| quote! { #param })
        .collect();
    let mut builder_struct_generic_consumers = info.generic_arguments();
    let mut builder_struct_fields = Vec::new();
    let mut builder_struct_field_names = Vec::new();

    for field in &info.fields {
        let field_name = &field.name;

        let arg_type = field.make_try_constructor_arg_type(info, builder_type)?;
        if let ArgType::Plain(plain_type) = arg_type {
            // No fancy builder function, we can just move the value directly into the struct.
            params.push(quote! { #field_name: #plain_type });
            builder_struct_fields.push(quote! { #field_name: #plain_type });
            builder_struct_field_names.push(quote! { #field_name });
            doc_table += &format!(
                "| `{}` | Directly pass in the value this field should contain |\n",
                field_name
            );
            if !field.self_referencing {
                if field.is_borrowed() {
                    head_recover_code[current_head_index] = quote! {
                        #field_name: ::ouroboros::macro_help::unbox(#field_name)
                    };
                } else {
                    head_recover_code[current_head_index] = quote! { #field_name };
                }
                current_head_index += 1;
            }
        } else if let ArgType::TraitBound(bound_type) = arg_type {
            // Trait bounds are much trickier. We need a special syntax to accept them in the
            // constructor, and generic parameters need to be added to the builder struct to make
            // it work.
            let builder_name = field.builder_name();
            params.push(quote! { #builder_name : impl #bound_type });
            // Ok so hear me out basically without this thing here my IDE thinks the rest of the
            // code is a string and it all turns green.
            {}
            doc_table += &format!(
                "| `{}` | Use a function or closure: `(",
                builder_name
            );
            let mut builder_args = Vec::new();
            for (index, borrow) in field.borrows.iter().enumerate() {
                let borrowed_name = &info.fields[borrow.index].name;
                builder_args.push(format_ident!("{}_illegal_static_reference", borrowed_name));
                doc_table += &format!(
                    "{}: &{}_",
                    borrowed_name,
                    if borrow.mutable { "mut " } else { "" },
                );
                if index < field.borrows.len() - 1 {
                    doc_table += ", ";
                }
            }
            doc_table += &format!(") -> Result<{}: _, Error_>` | \n", field_name);
            let builder_value = if builder_type.is_async() {
                quote! { #builder_name (#(#builder_args),*).await }
            } else {
                quote! { #builder_name (#(#builder_args),*) }
            };
            or_recover_code.push(quote! {
                let #field_name = match #builder_value {
                    ::core::result::Result::Ok(value) => value,
                    ::core::result::Result::Err(err)
                        => return ::core::result::Result::Err((err, Heads { #(#head_recover_code),* })),
                };
            });
            let generic_type_name =
                format_ident!("{}Builder_", to_class_case(field_name.to_string().as_str()));

            builder_struct_generic_producers.push(quote! { #generic_type_name: #bound_type });
            builder_struct_generic_consumers.push(quote! { #generic_type_name });
            builder_struct_fields.push(quote! { #builder_name: #generic_type_name });
            builder_struct_field_names.push(quote! { #builder_name });
        }
        if field.is_borrowed() {
            let boxed = field.boxed();
            if field.field_type == FieldType::BorrowedMut {
                or_recover_code.push(quote! { let mut #field_name = #boxed; });
            } else {
                or_recover_code.push(quote! { let #field_name = #boxed; });
            }
        }

        if field.field_type == FieldType::Borrowed {
            or_recover_code.push(field.make_illegal_static_reference());
        } else if field.field_type == FieldType::BorrowedMut {
            or_recover_code.push(field.make_illegal_static_mut_reference());
        }
    }
    let documentation = if !options.do_no_doc {
        let documentation = documentation + &doc_table;
        quote! {
            #[doc=#documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let or_recover_documentation = if !options.do_no_doc {
        let or_recover_documentation = or_recover_documentation + &doc_table;
        quote! {
            #[doc=#or_recover_documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let builder_documentation = if !options.do_no_doc {
        let builder_documentation = builder_documentation + &doc_table;
        quote! {
            #[doc=#builder_documentation]
        }
    } else {
        quote! { #[doc(hidden)] }
    };
    let or_recover_ident = match builder_type {
        BuilderType::AsyncSend => quote! { try_new_or_recover_async_send },
        BuilderType::Async => quote! { try_new_or_recover_async },
        BuilderType::Sync => quote! { try_new_or_recover },
    };
    let or_recover_constructor_fn = if builder_type.is_async() {
        quote! { async fn #or_recover_ident }
    } else {
        quote! { fn #or_recover_ident }
    };
    let constructor_fn = match builder_type {
        BuilderType::AsyncSend => quote! { async fn try_new_async_send },
        BuilderType::Async => quote! { async fn try_new_async },
        BuilderType::Sync => quote! { fn try_new },
    };
    let constructor_code = if builder_type.is_async() {
        quote! { #struct_name::#or_recover_ident(#(#builder_struct_field_names),*).await.map_err(|(error, _heads)| error) }
    } else {
        quote! { #struct_name::#or_recover_ident(#(#builder_struct_field_names),*).map_err(|(error, _heads)| error) }
    };
    let field_names: Vec<_> = info.fields.iter().map(|field| field.name.clone()).collect();
    let internal_ident = &info.internal_ident;
    let constructor_def = quote! {
        #documentation
        #visibility #constructor_fn<Error_>(#(#params),*) -> ::core::result::Result<#struct_name <#(#generic_args),*>, Error_> {
            #constructor_code
        }
        #or_recover_documentation
        #visibility #or_recover_constructor_fn<Error_>(#(#params),*) -> ::core::result::Result<#struct_name <#(#generic_args),*>, (Error_, Heads<#(#generic_args),*>)> {
            #(#or_recover_code)*
            ::core::result::Result::Ok(unsafe {
                Self {
                    actual_data: ::core::mem::MaybeUninit::new(#internal_ident {
                        #(#field_names),*
                    })
                }
            })
        }
    };
    builder_struct_generic_producers.push(quote! { Error_ });
    builder_struct_generic_consumers.push(quote! { Error_ });
    let generic_where = &info.generics.where_clause;
    let builder_fn = if builder_type.is_async() {
        quote! { async fn try_build }
    } else {
        quote! { fn try_build }
    };
    let or_recover_builder_fn = if builder_type.is_async() {
        quote! { async fn try_build_or_recover }
    } else {
        quote! { fn try_build_or_recover }
    };
    let builder_code = match builder_type {
        BuilderType::AsyncSend => quote! {
            #struct_name::try_new_async_send(
                #(self.#builder_struct_field_names),*
            ).await
        },
        BuilderType::Async => quote! {
            #struct_name::try_new_async(
                #(self.#builder_struct_field_names),*
            ).await
        },
        BuilderType::Sync => quote! {
            #struct_name::try_new(
                #(self.#builder_struct_field_names),*
            )
        },
    };
    let or_recover_builder_code = match builder_type {
        BuilderType::AsyncSend => quote! {
            #struct_name::try_new_or_recover_async_send(
                #(self.#builder_struct_field_names),*
            ).await
        },
        BuilderType::Async => quote! {
            #struct_name::try_new_or_recover_async(
                #(self.#builder_struct_field_names),*
            ).await
        },
        BuilderType::Sync => quote! {
            #struct_name::try_new_or_recover(
                #(self.#builder_struct_field_names),*
            )
        },
    };
    let builder_def = quote! {
        #builder_documentation
        #visibility struct #builder_struct_name <#(#builder_struct_generic_producers),*> #generic_where {
            #(#visibility #builder_struct_fields),*
        }
        impl<#(#builder_struct_generic_producers),*> #builder_struct_name <#(#builder_struct_generic_consumers),*> #generic_where {
            #[doc=#build_fn_documentation]
            #visibility #builder_fn(self) -> ::core::result::Result<#struct_name <#(#generic_args),*>, Error_> {
                #builder_code
            }
            #[doc=#build_or_recover_fn_documentation]
            #visibility #or_recover_builder_fn(self) -> ::core::result::Result<#struct_name <#(#generic_args),*>, (Error_, Heads<#(#generic_args),*>)> {
                #or_recover_builder_code
            }
        }
    };
    Ok((builder_struct_name, builder_def, constructor_def))
}
