// Copyright 2024 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, DataEnum, Error, Fields, Generics, Ident, Path};

use crate::{derive_try_from_bytes_inner, repr::EnumRepr, Trait};

/// Generates a tag enum for the given enum. This generates an enum with the
/// same non-align `repr`s, variants, and corresponding discriminants, but none
/// of the fields.
pub(crate) fn generate_tag_enum(repr: &EnumRepr, data: &DataEnum) -> TokenStream {
    let variants = data.variants.iter().map(|v| {
        let ident = &v.ident;
        if let Some((eq, discriminant)) = &v.discriminant {
            quote! { #ident #eq #discriminant }
        } else {
            quote! { #ident }
        }
    });

    // Don't include any `repr(align)` when generating the tag enum, as that
    // could add padding after the tag but before any variants, which is not the
    // correct behavior.
    let repr = match repr {
        EnumRepr::Transparent(span) => quote::quote_spanned! { *span => #[repr(transparent)] },
        EnumRepr::Compound(c, _) => quote! { #c },
    };

    quote! {
        #repr
        #[allow(dead_code, non_camel_case_types)]
        enum ___ZerocopyTag {
            #(#variants,)*
        }
    }
}

fn tag_ident(variant_ident: &Ident) -> Ident {
    Ident::new(&format!("___ZEROCOPY_TAG_{}", variant_ident), variant_ident.span())
}

/// Generates a constant for the tag associated with each variant of the enum.
/// When we match on the enum's tag, each arm matches one of these constants. We
/// have to use constants here because:
///
/// - The type that we're matching on is not the type of the tag, it's an
///   integer of the same size as the tag type and with the same bit patterns.
/// - We can't read the enum tag as an enum because the bytes may not represent
///   a valid variant.
/// - Patterns do not currently support const expressions, so we have to assign
///   these constants to names rather than use them inline in the `match`
///   statement.
fn generate_tag_consts(data: &DataEnum) -> TokenStream {
    let tags = data.variants.iter().map(|v| {
        let variant_ident = &v.ident;
        let tag_ident = tag_ident(variant_ident);

        quote! {
            // This casts the enum variant to its discriminant, and then
            // converts the discriminant to the target integral type via a
            // numeric cast [1].
            //
            // Because these are the same size, this is defined to be a no-op
            // and therefore is a lossless conversion [2].
            //
            // [1]: https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#enum-cast
            // [2]: https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#numeric-cast
            #[allow(non_upper_case_globals)]
            const #tag_ident: ___ZerocopyTagPrimitive =
                ___ZerocopyTag::#variant_ident as ___ZerocopyTagPrimitive;
        }
    });

    quote! {
        #(#tags)*
    }
}

fn variant_struct_ident(variant_ident: &Ident) -> Ident {
    Ident::new(&format!("___ZerocopyVariantStruct_{}", variant_ident), variant_ident.span())
}

/// Generates variant structs for the given enum variant.
///
/// These are structs associated with each variant of an enum. They are
/// `repr(C)` tuple structs with the same fields as the variant after a
/// `MaybeUninit<___ZerocopyInnerTag>`.
///
/// In order to unify the generated types for `repr(C)` and `repr(int)` enums,
/// we use a "fused" representation with fields for both an inner tag and an
/// outer tag. Depending on the repr, we will set one of these tags to the tag
/// type and the other to `()`. This lets us generate the same code but put the
/// tags in different locations.
fn generate_variant_structs(
    enum_name: &Ident,
    generics: &Generics,
    data: &DataEnum,
    zerocopy_crate: &Path,
) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // All variant structs have a `PhantomData<MyEnum<...>>` field because we
    // don't know which generic parameters each variant will use, and unused
    // generic parameters are a compile error.
    let phantom_ty = quote! {
        core_reexport::marker::PhantomData<#enum_name #ty_generics>
    };

    let variant_structs = data.variants.iter().filter_map(|variant| {
        // We don't generate variant structs for unit variants because we only
        // need to check the tag. This helps cut down our generated code a bit.
        if matches!(variant.fields, Fields::Unit) {
            return None;
        }

        let variant_struct_ident = variant_struct_ident(&variant.ident);
        let field_types = variant.fields.iter().map(|f| &f.ty);

        let variant_struct = parse_quote! {
            #[repr(C)]
            #[allow(non_snake_case)]
            struct #variant_struct_ident #impl_generics (
                core_reexport::mem::MaybeUninit<___ZerocopyInnerTag>,
                #(#field_types,)*
                #phantom_ty,
            ) #where_clause;
        };

        // We do this rather than emitting `#[derive(::zerocopy::TryFromBytes)]`
        // because that is not hygienic, and this is also more performant.
        let try_from_bytes_impl =
            derive_try_from_bytes_inner(&variant_struct, Trait::TryFromBytes, zerocopy_crate)
                .expect("derive_try_from_bytes_inner should not fail on synthesized type");

        Some(quote! {
            #variant_struct
            #try_from_bytes_impl
        })
    });

    quote! {
        #(#variant_structs)*
    }
}

fn generate_variants_union(generics: &Generics, data: &DataEnum) -> TokenStream {
    let (_, ty_generics, _) = generics.split_for_impl();

    let fields = data.variants.iter().filter_map(|variant| {
        // We don't generate variant structs for unit variants because we only
        // need to check the tag. This helps cut down our generated code a bit.
        if matches!(variant.fields, Fields::Unit) {
            return None;
        }

        // Field names are prefixed with `__field_` to prevent name collision with
        // the `__nonempty` field.
        let field_name = Ident::new(&format!("__field_{}", &variant.ident), variant.ident.span());
        let variant_struct_ident = variant_struct_ident(&variant.ident);

        Some(quote! {
            #field_name: core_reexport::mem::ManuallyDrop<
                #variant_struct_ident #ty_generics
            >,
        })
    });

    quote! {
        #[repr(C)]
        #[allow(non_snake_case)]
        union ___ZerocopyVariants #generics {
            #(#fields)*
            // Enums can have variants with no fields, but unions must
            // have at least one field. So we just add a trailing unit
            // to ensure that this union always has at least one field.
            // Because this union is `repr(C)`, this unit type does not
            // affect the layout.
            __nonempty: (),
        }
    }
}

/// Generates an implementation of `is_bit_valid` for an arbitrary enum.
///
/// The general process is:
///
/// 1. Generate a tag enum. This is an enum with the same repr, variants, and
///    corresponding discriminants as the original enum, but without any fields
///    on the variants. This gives us access to an enum where the variants have
///    the same discriminants as the one we're writing `is_bit_valid` for.
/// 2. Make constants from the variants of the tag enum. We need these because
///    we can't put const exprs in match arms.
/// 3. Generate variant structs. These are structs which have the same fields as
///    each variant of the enum, and are `#[repr(C)]` with an optional "inner
///    tag".
/// 4. Generate a variants union, with one field for each variant struct type.
/// 5. And finally, our raw enum is a `#[repr(C)]` struct of an "outer tag" and
///    the variants union.
///
/// See these reference links for fully-worked example decompositions.
///
/// - `repr(C)`: <https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields>
/// - `repr(int)`: <https://doc.rust-lang.org/reference/type-layout.html#primitive-representation-of-enums-with-fields>
/// - `repr(C, int)`: <https://doc.rust-lang.org/reference/type-layout.html#combining-primitive-representations-of-enums-with-fields-and-reprc>
pub(crate) fn derive_is_bit_valid(
    enum_ident: &Ident,
    repr: &EnumRepr,
    generics: &Generics,
    data: &DataEnum,
    zerocopy_crate: &Path,
) -> Result<TokenStream, Error> {
    let trait_path = Trait::TryFromBytes.crate_path(zerocopy_crate);
    let tag_enum = generate_tag_enum(repr, data);
    let tag_consts = generate_tag_consts(data);

    let (outer_tag_type, inner_tag_type) = if repr.is_c() {
        (quote! { ___ZerocopyTag }, quote! { () })
    } else if repr.is_primitive() {
        (quote! { () }, quote! { ___ZerocopyTag })
    } else {
        return Err(Error::new(
            Span::call_site(),
            "must have #[repr(C)] or #[repr(Int)] attribute in order to guarantee this type's memory layout",
        ));
    };

    let variant_structs = generate_variant_structs(enum_ident, generics, data, zerocopy_crate);
    let variants_union = generate_variants_union(generics, data);

    let (_, ty_generics, _) = generics.split_for_impl();

    let match_arms = data.variants.iter().map(|variant| {
        let tag_ident = tag_ident(&variant.ident);
        let variant_struct_ident = variant_struct_ident(&variant.ident);

        if matches!(variant.fields, Fields::Unit) {
            // Unit variants don't need any further validation beyond checking
            // the tag.
            quote! {
                #tag_ident => true
            }
        } else {
            quote! {
                #tag_ident => {
                    // SAFETY:
                    // - This cast is from a `repr(C)` union which has a field
                    //   of type `variant_struct_ident` to that variant struct
                    //   type itself. This addresses a subset of the bytes
                    //   addressed by `variants`.
                    // - The returned pointer is cast from `p`, and so has the
                    //   same provenance as `p`.
                    // - We checked that the tag of the enum matched the
                    //   constant for this variant, so this cast preserves
                    //   types and locations of all fields. Therefore, any
                    //   `UnsafeCell`s will have the same location as in the
                    //   original type.
                    let variant = unsafe {
                        variants.cast_unsized_unchecked(
                            |p: #zerocopy_crate::pointer::PtrInner<'_, ___ZerocopyVariants #ty_generics>| {
                                p.cast_sized::<#variant_struct_ident #ty_generics>()
                            }
                        )
                    };
                    // SAFETY: `cast_unsized_unchecked` removes the
                    // initialization invariant from `p`, so we re-assert that
                    // all of the bytes are initialized.
                    let variant = unsafe { variant.assume_initialized() };
                    <
                        #variant_struct_ident #ty_generics as #trait_path
                    >::is_bit_valid(variant)
                }
            }
        }
    });

    Ok(quote! {
        // SAFETY: We use `is_bit_valid` to validate that the bit pattern of the
        // enum's tag corresponds to one of the enum's discriminants. Then, we
        // check the bit validity of each field of the corresponding variant.
        // Thus, this is a sound implementation of `is_bit_valid`.
        fn is_bit_valid<___ZerocopyAliasing>(
            mut candidate: #zerocopy_crate::Maybe<'_, Self, ___ZerocopyAliasing>,
        ) -> #zerocopy_crate::util::macro_util::core_reexport::primitive::bool
        where
            ___ZerocopyAliasing: #zerocopy_crate::pointer::invariant::Reference,
        {
            use #zerocopy_crate::util::macro_util::core_reexport;

            #tag_enum

            type ___ZerocopyTagPrimitive = #zerocopy_crate::util::macro_util::SizeToTag<
                { core_reexport::mem::size_of::<___ZerocopyTag>() },
            >;

            #tag_consts

            type ___ZerocopyOuterTag = #outer_tag_type;
            type ___ZerocopyInnerTag = #inner_tag_type;

            #variant_structs

            #variants_union

            #[repr(C)]
            struct ___ZerocopyRawEnum #generics {
                tag: ___ZerocopyOuterTag,
                variants: ___ZerocopyVariants #ty_generics,
            }

            let tag = {
                // SAFETY:
                // - The provided cast addresses a subset of the bytes addressed
                //   by `candidate` because it addresses the starting tag of the
                //   enum.
                // - Because the pointer is cast from `candidate`, it has the
                //   same provenance as it.
                // - There are no `UnsafeCell`s in the tag because it is a
                //   primitive integer.
                let tag_ptr = unsafe {
                    candidate.reborrow().cast_unsized_unchecked(|p: #zerocopy_crate::pointer::PtrInner<'_, Self>| {
                        p.cast_sized::<___ZerocopyTagPrimitive>()
                    })
                };
                // SAFETY: `tag_ptr` is casted from `candidate`, whose referent
                // is `Initialized`. Since we have not written uninitialized
                // bytes into the referent, `tag_ptr` is also `Initialized`.
                let tag_ptr = unsafe { tag_ptr.assume_initialized() };
                tag_ptr.recall_validity::<_, (_, (_, _))>().read_unaligned::<#zerocopy_crate::BecauseImmutable>()
            };

            // SAFETY:
            // - The raw enum has the same fields in the same locations as the
            //   input enum, and may have a lower alignment. This guarantees
            //   that it addresses a subset of the bytes addressed by
            //   `candidate`.
            // - The returned pointer is cast from `p`, and so has the same
            //   provenance as `p`.
            // - The raw enum has the same types at the same locations as the
            //   original enum, and so preserves the locations of any
            //   `UnsafeCell`s.
            let raw_enum = unsafe {
                candidate.cast_unsized_unchecked(|p: #zerocopy_crate::pointer::PtrInner<'_, Self>| {
                    p.cast_sized::<___ZerocopyRawEnum #ty_generics>()
                })
            };
            // SAFETY: `cast_unsized_unchecked` removes the initialization
            // invariant from `p`, so we re-assert that all of the bytes are
            // initialized.
            let raw_enum = unsafe { raw_enum.assume_initialized() };
            // SAFETY:
            // - This projection returns a subfield of `this` using
            //   `addr_of_mut!`.
            // - Because the subfield pointer is derived from `this`, it has the
            //   same provenance.
            // - The locations of `UnsafeCell`s in the subfield match the
            //   locations of `UnsafeCell`s in `this`. This is because the
            //   subfield pointer just points to a smaller portion of the
            //   overall struct.
            let variants = unsafe {
                use #zerocopy_crate::pointer::PtrInner;
                raw_enum.cast_unsized_unchecked(|p: PtrInner<'_, ___ZerocopyRawEnum #ty_generics>| {
                    let p = p.as_non_null().as_ptr();
                    let ptr = core_reexport::ptr::addr_of_mut!((*p).variants);
                    // SAFETY: `ptr` is a projection into `p`, which is
                    // `NonNull`, and guaranteed not to wrap around the address
                    // space. Thus, `ptr` cannot be null.
                    let ptr = unsafe { core_reexport::ptr::NonNull::new_unchecked(ptr) };
                    unsafe { PtrInner::new(ptr) }
                })
            };

            #[allow(non_upper_case_globals)]
            match tag {
                #(#match_arms,)*
                _ => false,
            }
        }
    })
}
