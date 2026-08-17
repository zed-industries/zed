use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ExprPath, Fields, Ident, ItemStruct, Path, Type};

use crate::internals::{attributes::field, generics, schema};

/// function which computes derive output [proc_macro2::TokenStream]
/// of code, which computes declaration of a single field, which is later added to
/// the struct's definition as a whole  
fn field_declaration_output(
    field_name: Option<&Ident>,
    field_type: &Type,
    cratename: &Path,
    declaration_override: Option<ExprPath>,
) -> TokenStream2 {
    let default_path: ExprPath =
        syn::parse2(quote! { <#field_type as #cratename::BorshSchema>::declaration }).unwrap();

    let path = declaration_override.unwrap_or(default_path);

    if let Some(field_name) = field_name {
        let field_name = field_name.to_token_stream().to_string();
        quote! {
            (#field_name.to_string(), #path())
        }
    } else {
        quote! {
            #path()
        }
    }
}

/// function which computes derive output [proc_macro2::TokenStream]
/// of code, which adds definitions of a field to the output `definitions: &mut BTreeMap`
fn field_definitions_output(
    field_type: &Type,
    cratename: &Path,
    definitions_override: Option<ExprPath>,
) -> TokenStream2 {
    let default_path: ExprPath = syn::parse2(
        quote! { <#field_type as #cratename::BorshSchema>::add_definitions_recursively },
    )
    .unwrap();
    let path = definitions_override.unwrap_or(default_path);

    quote! {
        #path(definitions);
    }
}

pub fn process(input: &ItemStruct, cratename: Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let struct_name = name.to_token_stream().to_string();
    let generics = generics::without_defaults(&input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = generics::default_where(where_clause);
    let mut generics_output = schema::GenericsOutput::new(&generics);
    let (struct_fields, add_definitions_recursively) =
        process_fields(&cratename, &input.fields, &mut generics_output)?;

    let add_definitions_recursively = quote! {
        fn add_definitions_recursively(definitions: &mut #cratename::__private::maybestd::collections::BTreeMap<#cratename::schema::Declaration, #cratename::schema::Definition>) {
            #struct_fields
            let definition = #cratename::schema::Definition::Struct { fields };

            let no_recursion_flag = definitions.get(&<Self as #cratename::BorshSchema>::declaration()).is_none();
            #cratename::schema::add_definition(<Self as #cratename::BorshSchema>::declaration(), definition, definitions);
            if no_recursion_flag {
                #add_definitions_recursively
            }
        }
    };

    let (predicates, declaration) = generics_output.result(&struct_name, &cratename);
    where_clause.predicates.extend(predicates);
    Ok(quote! {
        impl #impl_generics #cratename::BorshSchema for #name #ty_generics #where_clause {
            fn declaration() -> #cratename::schema::Declaration {
                #declaration
            }
            #add_definitions_recursively
        }
    })
}

fn process_fields(
    cratename: &Path,
    fields: &Fields,
    generics: &mut schema::GenericsOutput,
) -> syn::Result<(TokenStream2, TokenStream2)> {
    let mut struct_fields = TokenStream2::new();
    let mut add_definitions_recursively = TokenStream2::new();

    // Generate function that returns the schema of required types.
    let mut fields_vec = vec![];
    schema::visit_struct_fields(fields, &mut generics.params_visitor)?;
    match fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                process_field(
                    field,
                    cratename,
                    &mut fields_vec,
                    &mut add_definitions_recursively,
                )?;
            }
            if !fields_vec.is_empty() {
                struct_fields = quote! {
                    let fields = #cratename::schema::Fields::NamedFields(#cratename::__private::maybestd::vec![#(#fields_vec),*]);
                };
            }
        }
        Fields::Unnamed(fields) => {
            for field in &fields.unnamed {
                process_field(
                    field,
                    cratename,
                    &mut fields_vec,
                    &mut add_definitions_recursively,
                )?;
            }
            if !fields_vec.is_empty() {
                struct_fields = quote! {
                    let fields = #cratename::schema::Fields::UnnamedFields(#cratename::__private::maybestd::vec![#(#fields_vec),*]);
                };
            }
        }
        Fields::Unit => {}
    }

    if fields_vec.is_empty() {
        struct_fields = quote! {
            let fields = #cratename::schema::Fields::Empty;
        };
    }
    Ok((struct_fields, add_definitions_recursively))
}
fn process_field(
    field: &syn::Field,
    cratename: &Path,
    fields_vec: &mut Vec<TokenStream2>,
    add_definitions_recursively: &mut TokenStream2,
) -> syn::Result<()> {
    let parsed = field::Attributes::parse(&field.attrs)?;
    if !parsed.skip {
        let field_name = field.ident.as_ref();
        let field_type = &field.ty;
        fields_vec.push(field_declaration_output(
            field_name,
            field_type,
            cratename,
            parsed.schema_declaration(),
        ));
        add_definitions_recursively.extend(field_definitions_output(
            field_type,
            cratename,
            parsed.schema_definitions(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::internals::test_helpers::{
        default_cratename, local_insta_assert_debug_snapshot, local_insta_assert_snapshot,
        pretty_print_syn_str,
    };

    use super::*;

    #[test]
    fn unit_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A;
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn wrapper_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<T>(T);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn tuple_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A(u64, String);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn tuple_struct_params() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K, V>(K, V);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn simple_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn simple_struct_with_custom_crate() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A {
                x: u64,
                y: String,
            }
        })
        .unwrap();

        let crate_: Path = syn::parse2(quote! { reexporter::borsh }).unwrap();
        let actual = process(&item_struct, crate_).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn simple_generics() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K, V> {
                x: HashMap<K, V>,
                y: String,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn trailing_comma_generics() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K, V>
            where
                K: Display + Debug,
            {
                x: HashMap<K, V>,
                y: String,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn tuple_struct_whole_skip() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A(#[borsh(skip)] String);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn tuple_struct_partial_skip() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A(#[borsh(skip)] u64, String);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_tuple_struct_borsh_skip1() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct G<K, V, U> (
                #[borsh(skip)]
                HashMap<K, V>,
                U,
            );
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_tuple_struct_borsh_skip2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct G<K, V, U> (
                HashMap<K, V>,
                #[borsh(skip)]
                U,
            );
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_tuple_struct_borsh_skip3() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct G<U, K, V> (
                #[borsh(skip)]
                HashMap<K, V>,
                U,
                K,
            );
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_tuple_struct_borsh_skip4() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct ASalad<C>(Tomatoes, #[borsh(skip)] C, Oil);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_named_fields_struct_borsh_skip() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct G<K, V, U> {
                #[borsh(skip)]
                x: HashMap<K, V>,
                y: U,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn recursive_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct CRecC {
                a: String,
                b: HashMap<String, CRecC>,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T: Debug>
            where
                T: TraitName,
            {
                field: T::Associated,
                another: V,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type_param_override() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params =
                    "T => <T as TraitName>::Associated"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type_param_override2() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(schema(params =
                    "T => T, T => <T as TraitName>::Associated"
               ))]
                field: (<T as TraitName>::Associated, T),
                another: V,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type_param_override_conflict() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<V, T>
            where
                T: TraitName,
            {
                #[borsh(skip,schema(params =
                    "T => <T as TraitName>::Associated"
               ))]
                field: <T as TraitName>::Associated,
                another: V,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename());

        local_insta_assert_debug_snapshot!(actual.unwrap_err());
    }

    #[test]
    fn check_with_funcs_skip_conflict() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K, V> {
                #[borsh(skip,schema(with_funcs(
                    declaration = "third_party_impl::declaration::<K, V>",
                    definitions = "third_party_impl::add_definitions_recursively::<K, V>"
                )))]
                x: ThirdParty<K, V>,
                y: u64,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename());

        local_insta_assert_debug_snapshot!(actual.unwrap_err());
    }

    #[test]
    fn with_funcs_attr() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K, V> {
                #[borsh(schema(with_funcs(
                    declaration = "third_party_impl::declaration::<K, V>",
                    definitions = "third_party_impl::add_definitions_recursively::<K, V>"
                )))]
                x: ThirdParty<K, V>,
                y: u64,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn schema_param_override3() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K: EntityRef, V> {
                #[borsh(
                    schema(
                        params = "V => V"
                    )
                )]
                x: PrimaryMap<K, V>,
                y: String,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }
}
