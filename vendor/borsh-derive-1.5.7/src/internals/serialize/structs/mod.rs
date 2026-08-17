use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Fields, ItemStruct, Path};

use crate::internals::{
    attributes::{field, BoundType},
    generics, serialize,
};

pub fn process(input: &ItemStruct, cratename: Path) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let generics = generics::without_defaults(&input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = generics::default_where(where_clause);
    let mut body = TokenStream2::new();
    let mut generics_output = serialize::GenericsOutput::new(&generics);
    match &input.fields {
        Fields::Named(fields) => {
            for field in &fields.named {
                let field_id = serialize::FieldId::Struct(field.ident.clone().unwrap());

                process_field(field, field_id, &cratename, &mut generics_output, &mut body)?;
            }
        }
        Fields::Unnamed(fields) => {
            for (field_idx, field) in fields.unnamed.iter().enumerate() {
                let field_id = serialize::FieldId::new_struct_unnamed(field_idx)?;

                process_field(field, field_id, &cratename, &mut generics_output, &mut body)?;
            }
        }
        Fields::Unit => {}
    }
    generics_output.extend(&mut where_clause, &cratename);

    Ok(quote! {
        impl #impl_generics #cratename::ser::BorshSerialize for #name #ty_generics #where_clause {
            fn serialize<__W: #cratename::io::Write>(&self, writer: &mut __W) -> ::core::result::Result<(), #cratename::io::Error> {
                #body
                Ok(())
            }
        }
    })
}

fn process_field(
    field: &syn::Field,
    field_id: serialize::FieldId,
    cratename: &Path,
    generics: &mut serialize::GenericsOutput,
    body: &mut TokenStream2,
) -> syn::Result<()> {
    let parsed = field::Attributes::parse(&field.attrs)?;
    let needs_bounds_derive = parsed.needs_bounds_derive(BoundType::Serialize);

    generics
        .overrides
        .extend(parsed.collect_bounds(BoundType::Serialize));
    if !parsed.skip {
        let delta = field_id.serialize_output(cratename, parsed.serialize_with);
        body.extend(delta);

        if needs_bounds_derive {
            generics.serialize_visitor.visit_field(field);
        }
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
    fn simple_generic_tuple_struct() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct TupleA<T>(T, u32);
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();
        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn bound_generics() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K: Key, V> where V: Value {
                x: HashMap<K, V>,
                y: String,
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
    fn generic_associated_type() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<T, V>
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
    fn generic_serialize_bound() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct C<T: Debug, U> {
                a: String,
                #[borsh(bound(serialize =
                    "T: borsh::ser::BorshSerialize + PartialOrd,
                     U: borsh::ser::BorshSerialize"
                ))]
                b: HashMap<T, U>,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn override_generic_associated_type_wrong_derive() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct Parametrized<T, V> where T: TraitName {
                #[borsh(bound(serialize =
                    "<T as TraitName>::Associated: borsh::ser::BorshSerialize"
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
    fn check_serialize_with_attr() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K: Ord, V> {
                #[borsh(serialize_with = "third_party_impl::serialize_third_party")]
                x: ThirdParty<K, V>,
                y: u64,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename()).unwrap();

        local_insta_assert_snapshot!(pretty_print_syn_str(&actual).unwrap());
    }

    #[test]
    fn check_serialize_with_skip_conflict() {
        let item_struct: ItemStruct = syn::parse2(quote! {
            struct A<K: Ord, V> {
                #[borsh(skip,serialize_with = "third_party_impl::serialize_third_party")]
                x: ThirdParty<K, V>,
                y: u64,
            }
        })
        .unwrap();

        let actual = process(&item_struct, default_cratename());

        let err = match actual {
            Ok(..) => unreachable!("expecting error here"),
            Err(err) => err,
        };
        local_insta_assert_debug_snapshot!(err);
    }
}
