use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::{Fields, Generics, Ident, ItemEnum, ItemStruct, Path, Variant, Visibility};

use crate::internals::{
    attributes::{field, item},
    enum_discriminant::Discriminants,
    generics, schema,
};

fn transform_variant_fields(mut input: Fields) -> Fields {
    match input {
        Fields::Named(ref mut named) => {
            for field in &mut named.named {
                let field_attrs = field::filter_attrs(field.attrs.drain(..)).collect::<Vec<_>>();
                field.attrs = field_attrs;
            }
        }
        Fields::Unnamed(ref mut unnamed) => {
            for field in &mut unnamed.unnamed {
                let field_attrs = field::filter_attrs(field.attrs.drain(..)).collect::<Vec<_>>();
                field.attrs = field_attrs;
            }
        }
        _ => {}
    }
    input
}

pub fn process(input: &ItemEnum, cratename: Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let enum_name = name.to_token_stream().to_string();
    let generics = generics::without_defaults(&input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = generics::default_where(where_clause);
    let mut generics_output = schema::GenericsOutput::new(&generics);
    let use_discriminant = item::contains_use_discriminant(input)?;
    let discriminants = Discriminants::new(&input.variants);

    // Generate functions that return the schema for variants.
    let mut variants_defs = vec![];
    let mut inner_defs = TokenStream2::new();
    let mut add_recursive_defs = TokenStream2::new();
    for (variant_idx, variant) in input.variants.iter().enumerate() {
        let discriminant_info = DiscriminantInfo {
            variant_idx,
            discriminants: &discriminants,
            use_discriminant,
        };
        let variant_output = process_variant(
            variant,
            discriminant_info,
            &cratename,
            &enum_name,
            &generics,
            &mut generics_output,
        )?;
        inner_defs.extend(variant_output.inner_struct);
        add_recursive_defs.extend(variant_output.add_definitions_recursively_call);
        variants_defs.push(variant_output.variant_entry);
    }

    let type_definitions = quote! {
        fn add_definitions_recursively(definitions: &mut #cratename::__private::maybestd::collections::BTreeMap<#cratename::schema::Declaration, #cratename::schema::Definition>) {
            #inner_defs
            #add_recursive_defs
            let definition = #cratename::schema::Definition::Enum {
                tag_width: 1,
                variants: #cratename::__private::maybestd::vec![#(#variants_defs),*],
            };
            #cratename::schema::add_definition(<Self as #cratename::BorshSchema>::declaration(), definition, definitions);
        }
    };

    let (predicates, declaration) = generics_output.result(&enum_name, &cratename);
    where_clause.predicates.extend(predicates);
    Ok(quote! {
        impl #impl_generics #cratename::BorshSchema for #name #ty_generics #where_clause {
            fn declaration() -> #cratename::schema::Declaration {
                #declaration
            }
            #type_definitions
        }
    })
}

struct VariantOutput {
    /// rust definition of the inner struct used in variant.
    inner_struct: TokenStream2,
    /// call to `add_definitions_recursively`.
    add_definitions_recursively_call: TokenStream2,
    /// entry with a variant's declaration, element in vector of whole enum's definition
    variant_entry: TokenStream2,
}

struct DiscriminantInfo<'a> {
    variant_idx: usize,
    discriminants: &'a Discriminants,
    use_discriminant: bool,
}

fn process_discriminant(
    variant_ident: &Ident,
    info: DiscriminantInfo<'_>,
) -> syn::Result<TokenStream2> {
    info.discriminants
        .get(variant_ident, info.use_discriminant, info.variant_idx)
}

fn process_variant(
    variant: &Variant,
    discriminant_info: DiscriminantInfo,
    cratename: &Path,
    enum_name: &str,
    enum_generics: &Generics,
    generics_output: &mut schema::GenericsOutput,
) -> syn::Result<VariantOutput> {
    let variant_name = variant.ident.to_token_stream().to_string();
    let full_variant_name = format!("{}{}", enum_name, variant_name);
    let full_variant_ident = Ident::new(&full_variant_name, Span::call_site());

    schema::visit_struct_fields(&variant.fields, &mut generics_output.params_visitor)?;
    let (inner_struct, inner_struct_generics) =
        inner_struct_definition(variant, cratename, &full_variant_ident, enum_generics);
    let (_ig, inner_struct_ty_generics, _wc) = inner_struct_generics.split_for_impl();

    let variant_type = quote! {
        <#full_variant_ident #inner_struct_ty_generics as #cratename::BorshSchema>
    };
    let discriminant_value = process_discriminant(&variant.ident, discriminant_info)?;

    Ok(VariantOutput {
        inner_struct,
        add_definitions_recursively_call: quote! {
            #variant_type::add_definitions_recursively(definitions);
        },
        variant_entry: quote! {
            (u8::from(#discriminant_value) as i64,
             #variant_name.into(),
             #variant_type::declaration())
        },
    })
}

fn inner_struct_definition(
    variant: &Variant,
    cratename: &Path,
    inner_struct_ident: &Ident,
    enum_generics: &Generics,
) -> (TokenStream2, Generics) {
    let transformed_fields = transform_variant_fields(variant.fields.clone());

    let mut variant_schema_params_visitor = generics::FindTyParams::new(enum_generics);
    schema::visit_struct_fields_unconditional(&variant.fields, &mut variant_schema_params_visitor);
    let variant_not_skipped_params = variant_schema_params_visitor
        .process_for_params()
        .into_iter()
        .collect::<HashSet<_>>();
    let inner_struct_generics =
        schema::filter_used_params(enum_generics, variant_not_skipped_params);

    let inner_struct = ItemStruct {
        attrs: vec![],
        vis: Visibility::Inherited,
        struct_token: Default::default(),
        ident: inner_struct_ident.clone(),
        generics: inner_struct_generics.clone(),
        fields: transformed_fields,
        semi_token: Some(Default::default()),
    };
    let crate_str = syn::LitStr::new(&cratename.to_token_stream().to_string(), Span::call_site());
    let inner_struct = quote! {
        #[allow(dead_code)]
        #[derive(#cratename::BorshSchema)]
        #[borsh(crate = #crate_str)]
        #inner_struct
    };
    (inner_struct, inner_struct_generics)
}

#[cfg(test)]
mod tests {
    use crate::internals::test_helpers::{
        default_cratename, local_insta_assert_debug_snapshot, local_insta_assert_snapshot,
        pretty_print_syn_str,
    };

    use super::*;

    #[test]
    fn simple_enum() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A {
                Bacon,
                Eggs
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn simple_enum_with_custom_crate() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A {
                Bacon,
                Eggs
            }
        })
        .unwrap();

        let crate_: Path = syn::parse2(quote! { reexporter::borsh }).unwrap();
        let actual = process(&item_enum, crate_).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn borsh_discriminant_false() {
        let item_enum: ItemEnum = syn::parse2(quote! {
           #[borsh(use_discriminant = false)]
            enum X {
                A,
                B = 20,
                C,
                D,
                E = 10,
                F,
            }
        })
        .unwrap();
        let actual = process(&item_enum, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }
    #[test]
    fn borsh_discriminant_true() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            #[borsh(use_discriminant = true)]
            enum X {
                A,
                B = 20,
                C,
                D,
                E = 10,
                F,
            }
        })
        .unwrap();
        let actual = process(&item_enum, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn single_field_enum() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A {
                Bacon,
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn complex_enum() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A {
                Bacon,
                Eggs,
                Salad(Tomatoes, Cucumber, Oil),
                Sausage{wrapper: Wrapper, filling: Filling},
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn complex_enum_generics() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A<C, W> {
                Bacon,
                Eggs,
                Salad(Tomatoes, C, Oil),
                Sausage{wrapper: W, filling: Filling},
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn trailing_comma_generics() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum Side<B, A>
            where
                A: Display + Debug,
                B: Display + Debug,
            {
                Left(A),
                Right(B),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn test_filter_foreign_attrs() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum A {
                #[serde(rename = "ab")]
                B {
                    #[serde(rename = "abc")]
                    c: i32,
                    #[borsh(skip)]
                    d: u32,
                    l: u64,
                },
                Negative {
                    beta: String,
                }
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn complex_enum_generics_borsh_skip_tuple_field() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A<C: Eq, W> where W: Hash {
                Bacon,
                Eggs,
                Salad(Tomatoes, #[borsh(skip)] C, Oil),
                Sausage{wrapper: W, filling: Filling},
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn complex_enum_generics_borsh_skip_named_field() {
        let item_enum: ItemEnum = syn::parse2(quote! {
            enum A<W, U, C> {
                Bacon,
                Eggs,
                Salad(Tomatoes, C, Oil),
                Sausage{
                    #[borsh(skip)]
                    wrapper: W,
                    filling: Filling,
                    unexpected: U,
                },
            }
        })
        .unwrap();

        let actual = process(&item_enum, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn recursive_enum() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum A<K: Key, V> where V: Value {
                B {
                    x: HashMap<K, V>,
                    y: String,
                },
                C(K, Vec<A>),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum EnumParametrized<T, K, V>
            where
                K: TraitName,
                K: core::cmp::Ord,
                V: core::cmp::Ord,
                T: Eq + Hash,
            {
                B {
                    x: BTreeMap<K, V>,
                    y: String,
                    z: K::Associated,
                },
                C(T, u16),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }
    #[test]
    fn generic_associated_type_param_override() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum EnumParametrized<T, K, V>
            where
                K: TraitName,
                K: core::cmp::Ord,
                V: core::cmp::Ord,
                T: Eq + Hash,
            {
                B {
                    x: BTreeMap<K, V>,
                    y: String,
                    #[borsh(schema(params = "K => <K as TraitName>::Associated"))]
                    z: <K as TraitName>::Associated,
                },
                C(T, u16),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn generic_associated_type_param_override_conflict() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum EnumParametrized<T, K, V>
            where
                K: TraitName,
            {
                B {
                    x: Vec<V>,
                    #[borsh(skip,schema(params = "K => <K as TraitName>::Associated"))]
                    z: <K as TraitName>::Associated,
                },
                C(T, u16),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename());

        local_insta_assert_debug_snapshot!(actual.unwrap_err());
    }

    #[test]
    fn check_with_funcs_skip_conflict() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum C<K, V> {
                C3(u64, u64),
                C4(
                    u64,
                    #[borsh(skip,schema(with_funcs(
                        declaration = "third_party_impl::declaration::<K, V>",
                        definitions = "third_party_impl::add_definitions_recursively::<K, V>"
                    )))]
                    ThirdParty<K, V>,
                ),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename());

        local_insta_assert_debug_snapshot!(actual.unwrap_err());
    }

    #[test]
    fn with_funcs_attr() {
        let item_struct: ItemEnum = syn::parse2(quote! {
            enum C<K, V> {
                C3(u64, u64),
                C4(
                    u64,
                    #[borsh(schema(with_funcs(
                        declaration = "third_party_impl::declaration::<K, V>",
                        definitions = "third_party_impl::add_definitions_recursively::<K, V>"
                    )))]
                    ThirdParty<K, V>,
                ),
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }
}
