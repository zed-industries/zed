#![allow(unused_imports)]
use std::{cmp, convert::TryFrom};

use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream, Parser},
  punctuated::Punctuated,
  spanned::Spanned,
  Result, *,
};

macro_rules! bail {
  ($msg:expr $(,)?) => {
    return Err(Error::new(Span::call_site(), &$msg[..]))
  };

  ( $msg:expr => $span_to_blame:expr $(,)? ) => {
    return Err(Error::new_spanned(&$span_to_blame, $msg))
  };
}

pub trait Derivable {
  fn ident(input: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path>;
  fn implies_trait(_crate_name: &TokenStream) -> Option<TokenStream> {
    None
  }
  fn asserts(
    _input: &DeriveInput, _crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    Ok(quote!())
  }
  fn check_attributes(_ty: &Data, _attributes: &[Attribute]) -> Result<()> {
    Ok(())
  }
  fn trait_impl(
    _input: &DeriveInput, _crate_name: &TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    Ok((quote!(), quote!()))
  }
  fn requires_where_clause() -> bool {
    true
  }
  fn explicit_bounds_attribute_name() -> Option<&'static str> {
    None
  }

  /// If this trait has a custom meaning for "perfect derive", this function
  /// should be overridden to return `Some`.
  ///
  /// The default is "the fields of a struct; unions and enums not supported".
  fn perfect_derive_fields(_input: &DeriveInput) -> Option<Fields> {
    None
  }
}

pub struct Pod;

impl Derivable for Pod {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::Pod))
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    let repr = get_repr(&input.attrs)?;

    let completly_packed =
      repr.packed == Some(1) || repr.repr == Repr::Transparent;

    if !completly_packed && !input.generics.params.is_empty() {
      bail!("\
        Pod requires cannot be derived for non-packed types containing \
        generic parameters because the padding requirements can't be verified \
        for generic non-packed structs\
      " => input.generics.params.first().unwrap());
    }

    match &input.data {
      Data::Struct(_) => {
        let assert_no_padding = if !completly_packed {
          Some(generate_assert_no_padding(input, None, "Pod")?)
        } else {
          None
        };
        let assert_fields_are_pod = generate_fields_are_trait(
          input,
          None,
          Self::ident(input, crate_name)?,
        )?;

        Ok(quote!(
          #assert_no_padding
          #assert_fields_are_pod
        ))
      }
      Data::Enum(_) => bail!("Deriving Pod is not supported for enums"),
      Data::Union(_) => bail!("Deriving Pod is not supported for unions"),
    }
  }

  fn check_attributes(_ty: &Data, attributes: &[Attribute]) -> Result<()> {
    let repr = get_repr(attributes)?;
    match repr.repr {
      Repr::C => Ok(()),
      Repr::Transparent => Ok(()),
      _ => {
        bail!("Pod requires the type to be #[repr(C)] or #[repr(transparent)]")
      }
    }
  }
}

pub struct AnyBitPattern;

impl Derivable for AnyBitPattern {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::AnyBitPattern))
  }

  fn implies_trait(crate_name: &TokenStream) -> Option<TokenStream> {
    Some(quote!(#crate_name::Zeroable))
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    match &input.data {
      Data::Union(_) => Ok(quote!()), // unions are always `AnyBitPattern`
      Data::Struct(_) => {
        generate_fields_are_trait(input, None, Self::ident(input, crate_name)?)
      }
      Data::Enum(_) => {
        bail!("Deriving AnyBitPattern is not supported for enums")
      }
    }
  }
}

pub struct Zeroable;

/// Helper function to get the variant with discriminant zero (implicit or
/// explicit).
fn get_zero_variant(enum_: &DataEnum) -> Result<Option<&Variant>> {
  let iter = VariantDiscriminantIterator::new(enum_.variants.iter());
  let mut zero_variant = None;
  for res in iter {
    let (discriminant, variant) = res?;
    if discriminant == 0 {
      zero_variant = Some(variant);
      break;
    }
  }
  Ok(zero_variant)
}

impl Derivable for Zeroable {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::Zeroable))
  }

  fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()> {
    let repr = get_repr(attributes)?;
    match ty {
      Data::Struct(_) => Ok(()),
      Data::Enum(_) => {
        if !matches!(
          repr.repr,
          Repr::C | Repr::Integer(_) | Repr::CWithDiscriminant(_)
        ) {
          bail!("Zeroable requires the enum to be an explicit #[repr(Int)] and/or #[repr(C)]")
        }

        // We ensure there is a zero variant in `asserts`, since it is needed
        // there anyway.

        Ok(())
      }
      Data::Union(_) => Ok(()),
    }
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    match &input.data {
      Data::Union(_) => Ok(quote!()), // unions are always `Zeroable`
      Data::Struct(_) => {
        generate_fields_are_trait(input, None, Self::ident(input, crate_name)?)
      }
      Data::Enum(enum_) => {
        let zero_variant = get_zero_variant(enum_)?;

        if zero_variant.is_none() {
          bail!("No variant's discriminant is 0")
        };

        generate_fields_are_trait(
          input,
          zero_variant,
          Self::ident(input, crate_name)?,
        )
      }
    }
  }

  fn explicit_bounds_attribute_name() -> Option<&'static str> {
    Some("zeroable")
  }

  fn perfect_derive_fields(input: &DeriveInput) -> Option<Fields> {
    match &input.data {
      Data::Struct(struct_) => Some(struct_.fields.clone()),
      Data::Enum(enum_) => {
        // We handle `Err` returns from `get_zero_variant` in `asserts`, so it's
        // fine to just ignore them here and return `None`.
        // Otherwise, we clone the `fields` of the zero variant (if any).
        Some(get_zero_variant(enum_).ok()??.fields.clone())
      }
      Data::Union(_) => None,
    }
  }
}

pub struct NoUninit;

impl Derivable for NoUninit {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::NoUninit))
  }

  fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()> {
    let repr = get_repr(attributes)?;
    match ty {
      Data::Struct(_) => match repr.repr {
        Repr::C | Repr::Transparent => Ok(()),
        _ => bail!("NoUninit requires the struct to be #[repr(C)] or #[repr(transparent)]"),
      },
      Data::Enum(DataEnum { variants,.. }) => {
        if !enum_has_fields(variants.iter()) {
          if matches!(repr.repr, Repr::C | Repr::Integer(_)) {
            Ok(())
          } else {
            bail!("NoUninit requires the enum to be #[repr(C)] or #[repr(Int)]")
          }
        } else if matches!(repr.repr, Repr::Rust) {
          bail!("NoUninit requires an explicit repr annotation because `repr(Rust)` doesn't have a specified type layout")
        } else {
          Ok(())
        }
      },
      Data::Union(_) => bail!("NoUninit can only be derived on enums and structs")
    }
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
      bail!("NoUninit cannot be derived for structs containing generic parameters because the padding requirements can't be verified for generic structs");
    }

    match &input.data {
      Data::Struct(DataStruct { .. }) => {
        let assert_no_padding =
          generate_assert_no_padding(&input, None, "NoUninit")?;
        let assert_fields_are_no_padding = generate_fields_are_trait(
          &input,
          None,
          Self::ident(input, crate_name)?,
        )?;

        Ok(quote!(
            #assert_no_padding
            #assert_fields_are_no_padding
        ))
      }
      Data::Enum(DataEnum { variants, .. }) => {
        if enum_has_fields(variants.iter()) {
          // There are two different C representations for enums with fields:
          // There's `#[repr(C)]`/`[repr(C, int)]` and `#[repr(int)]`.
          // `#[repr(C)]` is equivalent to a struct containing the discriminant
          // and a union of structs representing each variant's fields.
          // `#[repr(int)]` is equivalent to a union containing structs of the
          // discriminant and the fields.
          //
          // See https://doc.rust-lang.org/reference/type-layout.html#r-layout.repr.c.adt
          // and https://doc.rust-lang.org/reference/type-layout.html#r-layout.repr.primitive.adt
          //
          // In practice the only difference between the two is whether and
          // where padding bytes are placed. For `#[repr(C)]` enums, the first
          // enum fields of all variants start at the same location (the first
          // byte in the union). For `#[repr(int)]` enums, the structs
          // representing each variant are layed out individually and padding
          // does not depend on other variants, but only on the size of the
          // discriminant and the alignment of the first field. The location of
          // the first field might differ between variants, potentially
          // resulting in less padding or padding placed later in the enum.
          //
          // The `NoUninit` derive macro asserts that no padding exists by
          // removing all padding with `#[repr(packed)]` and checking that this
          // doesn't change the size. Since the location and presence of
          // padding bytes is the only difference between the two
          // representations and we're removing all padding bytes, the resuling
          // layout would identical for both representations. This means that
          // we can just pick one of the representations and don't have to
          // implement desugaring for both. We chose to implement the
          // desugaring for `#[repr(int)]`.

          let enum_discriminant = generate_enum_discriminant(input)?;
          let variant_assertions = variants
            .iter()
            .map(|variant| {
              let assert_no_padding =
                generate_assert_no_padding(&input, Some(variant), "NoUninit")?;
              let assert_fields_are_no_padding = generate_fields_are_trait(
                &input,
                Some(variant),
                Self::ident(input, crate_name)?,
              )?;

              Ok(quote!(
                  #assert_no_padding
                  #assert_fields_are_no_padding
              ))
            })
            .collect::<Result<Vec<_>>>()?;
          Ok(quote! {
            const _: () = {
              #enum_discriminant
              #(#variant_assertions)*
            };
          })
        } else {
          Ok(quote!())
        }
      }
      Data::Union(_) => bail!("NoUninit cannot be derived for unions"), /* shouldn't be possible since we already error in attribute check for this case */
    }
  }

  fn trait_impl(
    _input: &DeriveInput, _crate_name: &TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    Ok((quote!(), quote!()))
  }
}

pub struct CheckedBitPattern;

impl Derivable for CheckedBitPattern {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::CheckedBitPattern))
  }

  fn check_attributes(ty: &Data, attributes: &[Attribute]) -> Result<()> {
    let repr = get_repr(attributes)?;
    match ty {
      Data::Struct(_) => match repr.repr {
        Repr::C | Repr::Transparent => Ok(()),
        _ => bail!("CheckedBitPattern derive requires the struct to be #[repr(C)] or #[repr(transparent)]"),
      },
      Data::Enum(DataEnum { variants,.. }) => {
        if !enum_has_fields(variants.iter()){
          if matches!(repr.repr, Repr::C | Repr::Integer(_)) {
            Ok(())
          } else {
            bail!("CheckedBitPattern requires the enum to be #[repr(C)] or #[repr(Int)]")
          }
        } else if matches!(repr.repr, Repr::Rust) {
          bail!("CheckedBitPattern requires an explicit repr annotation because `repr(Rust)` doesn't have a specified type layout")
        } else {
          Ok(())
        }
      }
      Data::Union(_) => bail!("CheckedBitPattern can only be derived on enums and structs")
    }
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    if !input.generics.params.is_empty() {
      bail!("CheckedBitPattern cannot be derived for structs containing generic parameters");
    }

    match &input.data {
      Data::Struct(DataStruct { .. }) => {
        let assert_fields_are_maybe_pod = generate_fields_are_trait(
          &input,
          None,
          Self::ident(input, crate_name)?,
        )?;

        Ok(assert_fields_are_maybe_pod)
      }
      // nothing needed, already guaranteed OK by NoUninit.
      Data::Enum(_) => Ok(quote!()),
      Data::Union(_) => bail!("Internal error in CheckedBitPattern derive"), /* shouldn't be possible since we already error in attribute check for this case */
    }
  }

  fn trait_impl(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    match &input.data {
      Data::Struct(DataStruct { fields, .. }) => {
        generate_checked_bit_pattern_struct(
          &input.ident,
          fields,
          &input.attrs,
          crate_name,
        )
      }
      Data::Enum(DataEnum { variants, .. }) => {
        generate_checked_bit_pattern_enum(input, variants, crate_name)
      }
      Data::Union(_) => bail!("Internal error in CheckedBitPattern derive"), /* shouldn't be possible since we already error in attribute check for this case */
    }
  }
}

pub struct TransparentWrapper;

struct WrappedType {
  wrapped_type: syn::Type,
  /// Was the type given with a #[transparent(Type)] attribute.
  explicit: bool,
}

impl TransparentWrapper {
  fn get_wrapped_type(
    attributes: &[Attribute], fields: &Fields,
  ) -> Option<WrappedType> {
    let transparent_param =
      get_type_from_simple_attr(attributes, "transparent")
        .map(|wrapped_type| WrappedType { wrapped_type, explicit: true });
    transparent_param.or_else(|| {
      let mut types = get_field_types(&fields);
      let first_type = types.next();
      if let Some(_) = types.next() {
        // can't guess param type if there is more than one field
        return None;
      } else {
        first_type
          .cloned()
          .map(|wrapped_type| WrappedType { wrapped_type, explicit: false })
      }
    })
  }
}

impl Derivable for TransparentWrapper {
  fn ident(input: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    let fields = get_struct_fields(input)?;

    let WrappedType { wrapped_type: ty, .. } =
      match Self::get_wrapped_type(&input.attrs, &fields) {
        Some(ty) => ty,
        None => bail!("when deriving TransparentWrapper for a struct with more \
                       than one field, you need to specify the transparent field \
                       using #[transparent(T)]"),
      };

    Ok(syn::parse_quote!(#crate_name::TransparentWrapper<#ty>))
  }

  fn asserts(
    input: &DeriveInput, crate_name: &TokenStream,
  ) -> Result<TokenStream> {
    let (impl_generics, _ty_generics, where_clause) =
      input.generics.split_for_impl();
    let fields = get_struct_fields(input)?;
    let (wrapped_type, explicit) =
      match Self::get_wrapped_type(&input.attrs, &fields) {
        Some(WrappedType { wrapped_type, explicit }) => {
          (wrapped_type.to_token_stream().to_string(), explicit)
        }
        None => unreachable!(), /* other code will already reject this derive */
      };
    let mut wrapped_field_ty = None;
    let mut nonwrapped_field_tys = vec![];
    for field in fields.iter() {
      let field_ty = &field.ty;
      if field_ty.to_token_stream().to_string() == wrapped_type {
        if wrapped_field_ty.is_some() {
          if explicit {
            bail!("TransparentWrapper must have one field of the wrapped type. \
                   The type given in `#[transparent(Type)]` must match tokenwise \
                   with the type in the struct definition, not just be the same type. \
                   You may be able to use a type alias to work around this limitation.");
          } else {
            bail!("TransparentWrapper must have one field of the wrapped type");
          }
        }
        wrapped_field_ty = Some(field_ty);
      } else {
        nonwrapped_field_tys.push(field_ty);
      }
    }
    if let Some(wrapped_field_ty) = wrapped_field_ty {
      Ok(quote!(
        const _: () = {
          #[repr(transparent)]
          #[allow(clippy::multiple_bound_locations)]
          struct AssertWrappedIsWrapped #impl_generics((u8, ::core::marker::PhantomData<#wrapped_field_ty>), #(#nonwrapped_field_tys),*) #where_clause;
          fn assert_zeroable<Z: #crate_name::Zeroable>() {}
          #[allow(clippy::multiple_bound_locations)]
          fn check #impl_generics () #where_clause {
            #(
              assert_zeroable::<#nonwrapped_field_tys>();
            )*
          }
        };
      ))
    } else {
      bail!("TransparentWrapper must have one field of the wrapped type")
    }
  }

  fn check_attributes(_ty: &Data, attributes: &[Attribute]) -> Result<()> {
    let repr = get_repr(attributes)?;

    match repr.repr {
      Repr::Transparent => Ok(()),
      _ => {
        bail!(
          "TransparentWrapper requires the struct to be #[repr(transparent)]"
        )
      }
    }
  }

  fn requires_where_clause() -> bool {
    false
  }
}

pub struct Contiguous;

impl Derivable for Contiguous {
  fn ident(_: &DeriveInput, crate_name: &TokenStream) -> Result<syn::Path> {
    Ok(syn::parse_quote!(#crate_name::Contiguous))
  }

  fn trait_impl(
    input: &DeriveInput, _crate_name: &TokenStream,
  ) -> Result<(TokenStream, TokenStream)> {
    let repr = get_repr(&input.attrs)?;

    let integer_ty = if let Some(integer_ty) = repr.repr.as_integer() {
      integer_ty
    } else {
      bail!("Contiguous requires the enum to be #[repr(Int)]");
    };

    let variants = get_enum_variants(input)?;
    if enum_has_fields(variants.clone()) {
      return Err(Error::new_spanned(
        &input,
        "Only fieldless enums are supported",
      ));
    }

    let mut variants_with_discriminant =
      VariantDiscriminantIterator::new(variants);

    let (min, max, count) = variants_with_discriminant.try_fold(
      (i128::MAX, i128::MIN, 0),
      |(min, max, count), res| {
        let (discriminant, _variant) = res?;
        Ok::<_, Error>((
          i128::min(min, discriminant),
          i128::max(max, discriminant),
          count + 1,
        ))
      },
    )?;

    if max - min != count - 1 {
      bail! {
        "Contiguous requires the enum discriminants to be contiguous",
      }
    }

    let min_lit = LitInt::new(&format!("{}", min), input.span());
    let max_lit = LitInt::new(&format!("{}", max), input.span());

    // `from_integer` and `into_integer` are usually provided by the trait's
    // default implementation. We override this implementation because it
    // goes through `transmute_copy`, which can lead to inefficient assembly as seen in https://github.com/Lokathor/bytemuck/issues/175 .

    Ok((
      quote!(),
      quote! {
          type Int = #integer_ty;

          #[allow(clippy::missing_docs_in_private_items)]
          const MIN_VALUE: #integer_ty = #min_lit;

          #[allow(clippy::missing_docs_in_private_items)]
          const MAX_VALUE: #integer_ty = #max_lit;

          #[inline]
          fn from_integer(value: Self::Int) -> Option<Self> {
            #[allow(clippy::manual_range_contains)]
            if Self::MIN_VALUE <= value && value <= Self::MAX_VALUE {
              Some(unsafe { ::core::mem::transmute(value) })
            } else {
              None
            }
          }

          #[inline]
          fn into_integer(self) -> Self::Int {
              self as #integer_ty
          }
      },
    ))
  }
}

fn get_struct_fields(input: &DeriveInput) -> Result<&Fields> {
  if let Data::Struct(DataStruct { fields, .. }) = &input.data {
    Ok(fields)
  } else {
    bail!("deriving this trait is only supported for structs")
  }
}

/// Extract the `Fields` off a `DeriveInput`, or, in the `enum` case, off
/// those of the `enum_variant`, when provided (e.g., for `Zeroable`).
///
/// We purposely allow not providing an `enum_variant` for cases where
/// the caller wants to reject supporting `enum`s (e.g., `NoPadding`).
fn get_fields(
  input: &DeriveInput, enum_variant: Option<&Variant>,
) -> Result<Fields> {
  match &input.data {
    Data::Struct(DataStruct { fields, .. }) => Ok(fields.clone()),
    Data::Union(DataUnion { fields, .. }) => Ok(Fields::Named(fields.clone())),
    Data::Enum(_) => match enum_variant {
      Some(variant) => Ok(variant.fields.clone()),
      None => bail!("deriving this trait is not supported for enums"),
    },
  }
}

fn get_enum_variants<'a>(
  input: &'a DeriveInput,
) -> Result<impl Iterator<Item = &'a Variant> + Clone + 'a> {
  if let Data::Enum(DataEnum { variants, .. }) = &input.data {
    Ok(variants.iter())
  } else {
    bail!("deriving this trait is only supported for enums")
  }
}

fn get_field_types<'a>(
  fields: &'a Fields,
) -> impl Iterator<Item = &'a Type> + 'a {
  fields.iter().map(|field| &field.ty)
}

fn generate_checked_bit_pattern_struct(
  input_ident: &Ident, fields: &Fields, attrs: &[Attribute],
  crate_name: &TokenStream,
) -> Result<(TokenStream, TokenStream)> {
  let bits_ty = Ident::new(&format!("{}Bits", input_ident), input_ident.span());

  let repr = get_repr(attrs)?;

  let field_names = fields
    .iter()
    .enumerate()
    .map(|(i, field)| {
      field.ident.clone().unwrap_or_else(|| {
        Ident::new(&format!("field{}", i), input_ident.span())
      })
    })
    .collect::<Vec<_>>();
  let field_tys = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();

  let field_name = &field_names[..];
  let field_ty = &field_tys[..];

  Ok((
    quote! {
        #[doc = #GENERATED_TYPE_DOCUMENTATION]
        #repr
        #[derive(Clone, Copy, #crate_name::AnyBitPattern)]
        #[allow(missing_docs)]
        pub struct #bits_ty {
            #(#field_name: <#field_ty as #crate_name::CheckedBitPattern>::Bits,)*
        }

        #[allow(unexpected_cfgs)]
        const _: () = {
          #[cfg(not(target_arch = "spirv"))]
          impl ::core::fmt::Debug for #bits_ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
              let mut debug_struct = ::core::fmt::Formatter::debug_struct(f, ::core::stringify!(#bits_ty));
              #(::core::fmt::DebugStruct::field(&mut debug_struct, ::core::stringify!(#field_name), &{ self.#field_name });)*
              ::core::fmt::DebugStruct::finish(&mut debug_struct)
            }
          }
        };
    },
    quote! {
        type Bits = #bits_ty;

        #[inline]
        #[allow(clippy::double_comparisons, unused)]
        fn is_valid_bit_pattern(bits: &#bits_ty) -> bool {
            #(<#field_ty as #crate_name::CheckedBitPattern>::is_valid_bit_pattern(&{ bits.#field_name }) && )* true
        }
    },
  ))
}

fn generate_checked_bit_pattern_enum(
  input: &DeriveInput, variants: &Punctuated<Variant, Token![,]>,
  crate_name: &TokenStream,
) -> Result<(TokenStream, TokenStream)> {
  if enum_has_fields(variants.iter()) {
    generate_checked_bit_pattern_enum_with_fields(input, variants, crate_name)
  } else {
    generate_checked_bit_pattern_enum_without_fields(
      input, variants, crate_name,
    )
  }
}

fn generate_checked_bit_pattern_enum_without_fields(
  input: &DeriveInput, variants: &Punctuated<Variant, Token![,]>,
  crate_name: &TokenStream,
) -> Result<(TokenStream, TokenStream)> {
  let span = input.span();
  let mut variants_with_discriminant =
    VariantDiscriminantIterator::new(variants.iter());

  let (min, max, count) = variants_with_discriminant.try_fold(
    (i128::MAX, i128::MIN, 0),
    |(min, max, count), res| {
      let (discriminant, _variant) = res?;
      Ok::<_, Error>((
        i128::min(min, discriminant),
        i128::max(max, discriminant),
        count + 1,
      ))
    },
  )?;

  let check = if count == 0 {
    quote!(false)
  } else if max - min == count - 1 {
    // contiguous range
    let min_lit = LitInt::new(&format!("{}", min), span);
    let max_lit = LitInt::new(&format!("{}", max), span);

    quote!(*bits >= #min_lit && *bits <= #max_lit)
  } else {
    // not contiguous range, check for each
    let variant_discriminant_lits =
      VariantDiscriminantIterator::new(variants.iter())
        .map(|res| {
          let (discriminant, _variant) = res?;
          Ok(LitInt::new(&format!("{}", discriminant), span))
        })
        .collect::<Result<Vec<_>>>()?;

    // count is at least 1
    let first = &variant_discriminant_lits[0];
    let rest = &variant_discriminant_lits[1..];

    quote!(matches!(*bits, #first #(| #rest )*))
  };

  let (integer, defs) = get_enum_discriminant(input, crate_name)?;
  Ok((
    quote!(#defs),
    quote! {
        type Bits = #integer;

        #[inline]
        #[allow(clippy::double_comparisons)]
        fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
            #check
        }
    },
  ))
}

fn generate_checked_bit_pattern_enum_with_fields(
  input: &DeriveInput, variants: &Punctuated<Variant, Token![,]>,
  crate_name: &TokenStream,
) -> Result<(TokenStream, TokenStream)> {
  let representation = get_repr(&input.attrs)?;
  let vis = &input.vis;

  match representation.repr {
    Repr::Rust => unreachable!(),
    Repr::C | Repr::CWithDiscriminant(_) => {
      let (integer, defs) = get_enum_discriminant(input, crate_name)?;
      let input_ident = &input.ident;

      let bits_repr = Representation { repr: Repr::C, ..representation };

      // the enum manually re-configured as the actual tagged union it
      // represents, thus circumventing the requirements rust imposes on
      // the tag even when using #[repr(C)] enum layout
      // see: https://doc.rust-lang.org/reference/type-layout.html#reprc-enums-with-fields
      let bits_ty_ident =
        Ident::new(&format!("{input_ident}Bits"), input.span());

      // the variants union part of the tagged union. These get put into a union
      // which gets the AnyBitPattern derive applied to it, thus checking
      // that the fields of the union obey the requriements of AnyBitPattern.
      // The types that actually go in the union are one more level of
      // indirection deep: we generate new structs for each variant
      // (`variant_struct_definitions`) which themselves have the
      // `CheckedBitPattern` derive applied, thus generating
      // `{variant_struct_ident}Bits` structs, which are the ones that go
      // into this union.
      let variants_union_ident =
        Ident::new(&format!("{}Variants", input.ident), input.span());

      let variant_struct_idents = variants.iter().map(|v| {
        Ident::new(&format!("{input_ident}Variant{}", v.ident), v.span())
      });

      let variant_struct_definitions =
        variant_struct_idents.clone().zip(variants.iter()).map(|(variant_struct_ident, v)| {
          let fields = v.fields.iter().map(|v| &v.ty);

          quote! {
            #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::CheckedBitPattern)]
            #[repr(C)]
            #vis struct #variant_struct_ident(#(#fields),*);
          }
        });

      let union_fields = variant_struct_idents
        .clone()
        .zip(variants.iter())
        .map(|(variant_struct_ident, v)| {
          let variant_struct_bits_ident =
            Ident::new(&format!("{variant_struct_ident}Bits"), input.span());
          let field_ident = &v.ident;
          quote! {
            #field_ident: #variant_struct_bits_ident
          }
        });

      let variant_checks = variant_struct_idents
        .clone()
        .zip(VariantDiscriminantIterator::new(variants.iter()))
        .zip(variants.iter())
        .map(|((variant_struct_ident, discriminant), v)| -> Result<_> {
          let (discriminant, _variant) = discriminant?;
          let discriminant = LitInt::new(&discriminant.to_string(), v.span());
          let ident = &v.ident;
          Ok(quote! {
            #discriminant => {
              let payload = unsafe { &bits.payload.#ident };
              <#variant_struct_ident as #crate_name::CheckedBitPattern>::is_valid_bit_pattern(payload)
            }
          })
        })
        .collect::<Result<Vec<_>>>()?;

      Ok((
        quote! {
          #defs

          #[doc = #GENERATED_TYPE_DOCUMENTATION]
          #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::AnyBitPattern)]
          #bits_repr
          #vis struct #bits_ty_ident {
            tag: #integer,
            payload: #variants_union_ident,
          }

          #[allow(unexpected_cfgs)]
          const _: () = {
            #[cfg(not(target_arch = "spirv"))]
            impl ::core::fmt::Debug for #bits_ty_ident {
              fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut debug_struct = ::core::fmt::Formatter::debug_struct(f, ::core::stringify!(#bits_ty_ident));
                ::core::fmt::DebugStruct::field(&mut debug_struct, "tag", &self.tag);
                ::core::fmt::DebugStruct::field(&mut debug_struct, "payload", &self.payload);
                ::core::fmt::DebugStruct::finish(&mut debug_struct)
              }
            }
          };

          #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::AnyBitPattern)]
          #[repr(C)]
          #[allow(non_snake_case)]
          #vis union #variants_union_ident {
            #(#union_fields,)*
          }

          #[allow(unexpected_cfgs)]
          const _: () = {
            #[cfg(not(target_arch = "spirv"))]
            impl ::core::fmt::Debug for #variants_union_ident {
              fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut debug_struct = ::core::fmt::Formatter::debug_struct(f, ::core::stringify!(#variants_union_ident));
                ::core::fmt::DebugStruct::finish_non_exhaustive(&mut debug_struct)
              }
            }
          };

          #(#variant_struct_definitions)*
        },
        quote! {
          type Bits = #bits_ty_ident;

          #[inline]
          #[allow(clippy::double_comparisons)]
          fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
            match bits.tag {
              #(#variant_checks)*
              _ => false,
            }
          }
        },
      ))
    }
    Repr::Transparent => {
      if variants.len() != 1 {
        bail!("enums with more than one variant cannot be transparent")
      }

      let variant = &variants[0];

      let bits_ty = Ident::new(&format!("{}Bits", input.ident), input.span());
      let fields = variant.fields.iter().map(|v| &v.ty);

      Ok((
        quote! {
          #[doc = #GENERATED_TYPE_DOCUMENTATION]
          #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::CheckedBitPattern)]
          #[repr(C)]
          #vis struct #bits_ty(#(#fields),*);
        },
        quote! {
          type Bits = <#bits_ty as #crate_name::CheckedBitPattern>::Bits;

          #[inline]
          #[allow(clippy::double_comparisons)]
          fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
            <#bits_ty as #crate_name::CheckedBitPattern>::is_valid_bit_pattern(bits)
          }
        },
      ))
    }
    Repr::Integer(integer) => {
      let bits_repr = Representation { repr: Repr::C, ..representation };
      let input_ident = &input.ident;

      // the enum manually re-configured as the union it represents. such a
      // union is the union of variants as a repr(c) struct with the
      // discriminator type inserted at the beginning. in our case we
      // union the `Bits` representation of each variant rather than the variant
      // itself, which we generate via a nested `CheckedBitPattern` derive
      // on the `variant_struct_definitions` generated below.
      //
      // see: https://doc.rust-lang.org/reference/type-layout.html#primitive-representation-of-enums-with-fields
      let bits_ty_ident =
        Ident::new(&format!("{input_ident}Bits"), input.span());

      let variant_struct_idents = variants.iter().map(|v| {
        Ident::new(&format!("{input_ident}Variant{}", v.ident), v.span())
      });

      let variant_struct_definitions =
        variant_struct_idents.clone().zip(variants.iter()).map(|(variant_struct_ident, v)| {
          let fields = v.fields.iter().map(|v| &v.ty);

          // adding the discriminant repr integer as first field, as described above
          quote! {
            #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::CheckedBitPattern)]
            #[repr(C)]
            #vis struct #variant_struct_ident(#integer, #(#fields),*);
          }
        });

      let union_fields = variant_struct_idents
        .clone()
        .zip(variants.iter())
        .map(|(variant_struct_ident, v)| {
          let variant_struct_bits_ident =
            Ident::new(&format!("{variant_struct_ident}Bits"), input.span());
          let field_ident = &v.ident;
          quote! {
            #field_ident: #variant_struct_bits_ident
          }
        });

      let variant_checks = variant_struct_idents
        .clone()
        .zip(VariantDiscriminantIterator::new(variants.iter()))
        .zip(variants.iter())
        .map(|((variant_struct_ident, discriminant), v)| -> Result<_> {
          let (discriminant, _variant) = discriminant?;
          let discriminant = LitInt::new(&discriminant.to_string(), v.span());
          let ident = &v.ident;
          Ok(quote! {
            #discriminant => {
              let payload = unsafe { &bits.#ident };
              <#variant_struct_ident as #crate_name::CheckedBitPattern>::is_valid_bit_pattern(payload)
            }
          })
        })
        .collect::<Result<Vec<_>>>()?;

      Ok((
        quote! {
          #[doc = #GENERATED_TYPE_DOCUMENTATION]
          #[derive(::core::clone::Clone, ::core::marker::Copy, #crate_name::AnyBitPattern)]
          #bits_repr
          #[allow(non_snake_case)]
          #vis union #bits_ty_ident {
            __tag: #integer,
            #(#union_fields,)*
          }

          #[allow(unexpected_cfgs)]
          const _: () = {
            #[cfg(not(target_arch = "spirv"))]
            impl ::core::fmt::Debug for #bits_ty_ident {
              fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                let mut debug_struct = ::core::fmt::Formatter::debug_struct(f, ::core::stringify!(#bits_ty_ident));
                ::core::fmt::DebugStruct::field(&mut debug_struct, "tag", unsafe { &self.__tag });
                ::core::fmt::DebugStruct::finish_non_exhaustive(&mut debug_struct)
              }
            }
          };

          #(#variant_struct_definitions)*
        },
        quote! {
          type Bits = #bits_ty_ident;

          #[inline]
          #[allow(clippy::double_comparisons)]
          fn is_valid_bit_pattern(bits: &Self::Bits) -> bool {
            match unsafe { bits.__tag } {
              #(#variant_checks)*
              _ => false,
            }
          }
        },
      ))
    }
  }
}

/// Check that a struct or enum has no padding by asserting that the size of
/// the type is equal to the sum of the size of it's fields and discriminant
/// (for enums, this must be asserted for each variant).
fn generate_assert_no_padding(
  input: &DeriveInput, enum_variant: Option<&Variant>, for_trait: &str,
) -> Result<TokenStream> {
  let struct_type = &input.ident;
  let fields = get_fields(input, enum_variant)?;

  // If the type is an enum, determine the type of its discriminant.
  let enum_discriminant = if matches!(input.data, Data::Enum(_)) {
    let ident =
      Ident::new(&format!("{}Discriminant", input.ident), input.ident.span());
    Some(ident.into_token_stream())
  } else {
    None
  };

  // Prepend the type of the discriminant to the types of the fields.
  let mut field_types = enum_discriminant
    .into_iter()
    .chain(get_field_types(&fields).map(ToTokens::to_token_stream));
  let size_sum = if let Some(first) = field_types.next() {
    let size_first = quote!(::core::mem::size_of::<#first>());
    let size_rest = quote!(#( + ::core::mem::size_of::<#field_types>() )*);

    quote!(#size_first #size_rest)
  } else {
    quote!(0)
  };

  let message =
    format!("derive({for_trait}) was applied to a type with padding");

  Ok(quote! {const _: () = {
    assert!(
        ::core::mem::size_of::<#struct_type>() == (#size_sum),
        #message,
    );
  };})
}

/// Check that all fields implement a given trait
fn generate_fields_are_trait(
  input: &DeriveInput, enum_variant: Option<&Variant>, trait_: syn::Path,
) -> Result<TokenStream> {
  let (impl_generics, _ty_generics, where_clause) =
    input.generics.split_for_impl();
  let fields = get_fields(input, enum_variant)?;
  let field_types = get_field_types(&fields);
  Ok(quote! {#(const _: fn() = || {
      #[allow(clippy::missing_const_for_fn)]
      #[doc(hidden)]
      fn check #impl_generics () #where_clause {
        fn assert_impl<T: #trait_>() {}
        assert_impl::<#field_types>();
      }
    };)*
  })
}

/// Get the type of an enum's discriminant.
///
/// For `repr(int)` and `repr(C, int)` enums, this will return the known bare
/// integer type specified.
///
/// For `repr(C)` enums, this will extract the underlying size chosen by rustc.
/// It will return a token stream which is a type expression that evaluates to
/// a primitive integer type of this size, using our `EnumTagIntegerBytes`
/// trait.
///
/// For fieldless `repr(C)` enums, we can feed the size of the enum directly
/// into the trait.
///
/// For `repr(C)` enums with fields, we generate a new fieldless `repr(C)` enum
/// with the same variants, then use that in the calculation. This is the
/// specified behavior, see https://doc.rust-lang.org/stable/reference/type-layout.html#reprc-enums-with-fields
///
/// Returns a tuple of (type ident, auxiliary definitions)
fn get_enum_discriminant(
  input: &DeriveInput, crate_name: &TokenStream,
) -> Result<(TokenStream, TokenStream)> {
  let repr = get_repr(&input.attrs)?;
  match repr.repr {
    Repr::C => {
      let e = if let Data::Enum(e) = &input.data { e } else { unreachable!() };
      if enum_has_fields(e.variants.iter()) {
        // If the enum has fields, we must first isolate the discriminant by
        // removing all the fields.
        let enum_discriminant = generate_enum_discriminant(input)?;
        let discriminant_ident = Ident::new(
          &format!("{}Discriminant", input.ident),
          input.ident.span(),
        );
        Ok((
          quote!(<[::core::primitive::u8; ::core::mem::size_of::<#discriminant_ident>()] as #crate_name::derive::EnumTagIntegerBytes>::Integer),
          quote! {
            #enum_discriminant
          },
        ))
      } else {
        // If the enum doesn't have fields, we can just use it directly.
        let ident = &input.ident;
        Ok((
          quote!(<[::core::primitive::u8; ::core::mem::size_of::<#ident>()] as #crate_name::derive::EnumTagIntegerBytes>::Integer),
          quote!(),
        ))
      }
    }
    Repr::Integer(integer) | Repr::CWithDiscriminant(integer) => {
      Ok((quote!(#integer), quote!()))
    }
    _ => unreachable!(),
  }
}

fn generate_enum_discriminant(input: &DeriveInput) -> Result<TokenStream> {
  let e = if let Data::Enum(e) = &input.data { e } else { unreachable!() };
  let repr = get_repr(&input.attrs)?;
  let repr = match repr.repr {
    Repr::C => quote!(#[repr(C)]),
    Repr::Integer(int) | Repr::CWithDiscriminant(int) => quote!(#[repr(#int)]),
    Repr::Rust | Repr::Transparent => unreachable!(),
  };
  let ident =
    Ident::new(&format!("{}Discriminant", input.ident), input.ident.span());
  let variants = e.variants.iter().cloned().map(|mut e| {
    e.fields = Fields::Unit;
    e
  });
  Ok(quote! {
    #repr
    #[allow(dead_code)]
    enum #ident {
      #(#variants,)*
    }
  })
}

fn get_wrapped_type_from_stream(tokens: TokenStream) -> Option<syn::Type> {
  let mut tokens = tokens.into_iter().peekable();
  match tokens.peek() {
    Some(TokenTree::Group(group)) => {
      let res = get_wrapped_type_from_stream(group.stream());
      tokens.next(); // remove the peeked token tree
      match tokens.next() {
        // If there were more tokens, the input was invalid
        Some(_) => None,
        None => res,
      }
    }
    _ => syn::parse2(tokens.collect()).ok(),
  }
}

/// get a simple `#[foo(bar)]` attribute, returning `bar`
fn get_type_from_simple_attr(
  attributes: &[Attribute], attr_name: &str,
) -> Option<syn::Type> {
  for attr in attributes {
    if let (AttrStyle::Outer, Meta::List(list)) = (&attr.style, &attr.meta) {
      if list.path.is_ident(attr_name) {
        if let Some(ty) = get_wrapped_type_from_stream(list.tokens.clone()) {
          return Some(ty);
        }
      }
    }
  }

  None
}

fn get_repr(attributes: &[Attribute]) -> Result<Representation> {
  attributes
    .iter()
    .filter_map(|attr| {
      if attr.path().is_ident("repr") {
        Some(attr.parse_args::<Representation>())
      } else {
        None
      }
    })
    .try_fold(Representation::default(), |a, b| {
      let b = b?;
      Ok(Representation {
        repr: match (a.repr, b.repr) {
          (a, Repr::Rust) => a,
          (Repr::Rust, b) => b,
          _ => bail!("conflicting representation hints"),
        },
        packed: match (a.packed, b.packed) {
          (a, None) => a,
          (None, b) => b,
          _ => bail!("conflicting representation hints"),
        },
        align: match (a.align, b.align) {
          (Some(a), Some(b)) => Some(cmp::max(a, b)),
          (a, None) => a,
          (None, b) => b,
        },
      })
    })
}

mk_repr! {
  U8 => u8,
  I8 => i8,
  U16 => u16,
  I16 => i16,
  U32 => u32,
  I32 => i32,
  U64 => u64,
  I64 => i64,
  I128 => i128,
  U128 => u128,
  Usize => usize,
  Isize => isize,
}
// where
macro_rules! mk_repr {(
  $(
    $Xn:ident => $xn:ident
  ),* $(,)?
) => (
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  enum IntegerRepr {
    $($Xn),*
  }

  impl<'a> TryFrom<&'a str> for IntegerRepr {
    type Error = &'a str;

    fn try_from(value: &'a str) -> std::result::Result<Self, &'a str> {
      match value {
        $(
          stringify!($xn) => Ok(Self::$Xn),
        )*
        _ => Err(value),
      }
    }
  }

  impl ToTokens for IntegerRepr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      match self {
        $(
          Self::$Xn => tokens.extend(quote!($xn)),
        )*
      }
    }
  }
)}
use mk_repr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Repr {
  Rust,
  C,
  Transparent,
  Integer(IntegerRepr),
  CWithDiscriminant(IntegerRepr),
}

impl Repr {
  fn as_integer(&self) -> Option<IntegerRepr> {
    if let Self::Integer(v) = self {
      Some(*v)
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Representation {
  packed: Option<u32>,
  align: Option<u32>,
  repr: Repr,
}

impl Default for Representation {
  fn default() -> Self {
    Self { packed: None, align: None, repr: Repr::Rust }
  }
}

impl Parse for Representation {
  fn parse(input: ParseStream<'_>) -> Result<Representation> {
    let mut ret = Representation::default();
    while !input.is_empty() {
      let keyword = input.parse::<Ident>()?;
      // preëmptively call `.to_string()` *once* (rather than on `is_ident()`)
      let keyword_str = keyword.to_string();
      let new_repr = match keyword_str.as_str() {
        "C" => Repr::C,
        "transparent" => Repr::Transparent,
        "packed" => {
          ret.packed = Some(if input.peek(token::Paren) {
            let contents;
            parenthesized!(contents in input);
            LitInt::base10_parse::<u32>(&contents.parse()?)?
          } else {
            1
          });
          let _: Option<Token![,]> = input.parse()?;
          continue;
        }
        "align" => {
          let contents;
          parenthesized!(contents in input);
          let new_align = LitInt::base10_parse::<u32>(&contents.parse()?)?;
          ret.align = Some(
            ret
              .align
              .map_or(new_align, |old_align| cmp::max(old_align, new_align)),
          );
          let _: Option<Token![,]> = input.parse()?;
          continue;
        }
        ident => {
          let primitive = IntegerRepr::try_from(ident)
            .map_err(|_| input.error("unrecognized representation hint"))?;
          Repr::Integer(primitive)
        }
      };
      ret.repr = match (ret.repr, new_repr) {
        (Repr::Rust, new_repr) => {
          // This is the first explicit repr.
          new_repr
        }
        (Repr::C, Repr::Integer(integer))
        | (Repr::Integer(integer), Repr::C) => {
          // Both the C repr and an integer repr have been specified
          // -> merge into a C wit discriminant.
          Repr::CWithDiscriminant(integer)
        }
        (_, _) => {
          return Err(input.error("duplicate representation hint"));
        }
      };
      let _: Option<Token![,]> = input.parse()?;
    }
    Ok(ret)
  }
}

impl ToTokens for Representation {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let mut meta = Punctuated::<_, Token![,]>::new();

    match self.repr {
      Repr::Rust => {}
      Repr::C => meta.push(quote!(C)),
      Repr::Transparent => meta.push(quote!(transparent)),
      Repr::Integer(primitive) => meta.push(quote!(#primitive)),
      Repr::CWithDiscriminant(primitive) => {
        meta.push(quote!(C));
        meta.push(quote!(#primitive));
      }
    }

    if let Some(packed) = self.packed.as_ref() {
      let lit = LitInt::new(&packed.to_string(), Span::call_site());
      meta.push(quote!(packed(#lit)));
    }

    if let Some(align) = self.align.as_ref() {
      let lit = LitInt::new(&align.to_string(), Span::call_site());
      meta.push(quote!(align(#lit)));
    }

    tokens.extend(quote!(
      #[repr(#meta)]
    ));
  }
}

fn enum_has_fields<'a>(
  mut variants: impl Iterator<Item = &'a Variant>,
) -> bool {
  variants.any(|v| matches!(v.fields, Fields::Named(_) | Fields::Unnamed(_)))
}

struct VariantDiscriminantIterator<'a, I: Iterator<Item = &'a Variant> + 'a> {
  inner: I,
  last_value: i128,
}

impl<'a, I: Iterator<Item = &'a Variant> + 'a>
  VariantDiscriminantIterator<'a, I>
{
  fn new(inner: I) -> Self {
    VariantDiscriminantIterator { inner, last_value: -1 }
  }
}

impl<'a, I: Iterator<Item = &'a Variant> + 'a> Iterator
  for VariantDiscriminantIterator<'a, I>
{
  type Item = Result<(i128, &'a Variant)>;

  fn next(&mut self) -> Option<Self::Item> {
    let variant = self.inner.next()?;

    if let Some((_, discriminant)) = &variant.discriminant {
      let discriminant_value = match parse_int_expr(discriminant) {
        Ok(value) => value,
        Err(e) => return Some(Err(e)),
      };
      self.last_value = discriminant_value;
    } else {
      // If this wraps, then either:
      // 1. the enum is using repr(u128), so wrapping is correct
      // 2. the enum is using repr(i<=128 or u<128), so the compiler will
      //    already emit a "wrapping discriminant" E0370 error.
      self.last_value = self.last_value.wrapping_add(1);
      // Static assert that there is no integer repr > 128 bits. If that
      // changes, the above comment is inaccurate and needs to be updated!
      // FIXME(zachs18): maybe should also do something to ensure `isize::BITS
      // <= 128`?
      if let Some(repr) = None::<IntegerRepr> {
        match repr {
          IntegerRepr::U8
          | IntegerRepr::I8
          | IntegerRepr::U16
          | IntegerRepr::I16
          | IntegerRepr::U32
          | IntegerRepr::I32
          | IntegerRepr::U64
          | IntegerRepr::I64
          | IntegerRepr::I128
          | IntegerRepr::U128
          | IntegerRepr::Usize
          | IntegerRepr::Isize => (),
        }
      }
    }

    Some(Ok((self.last_value, variant)))
  }
}

fn parse_int_expr(expr: &Expr) -> Result<i128> {
  match expr {
    Expr::Unary(ExprUnary { op: UnOp::Neg(_), expr, .. }) => {
      parse_int_expr(expr).map(|int| -int)
    }
    Expr::Lit(ExprLit { lit: Lit::Int(int), .. }) => int.base10_parse(),
    Expr::Lit(ExprLit { lit: Lit::Byte(byte), .. }) => Ok(byte.value().into()),
    _ => bail!("Not an integer expression"),
  }
}

#[cfg(test)]
mod tests {
  use syn::parse_quote;

  use super::{get_repr, IntegerRepr, Repr, Representation};

  #[test]
  fn parse_basic_repr() {
    let attr = parse_quote!(#[repr(C)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { repr: Repr::C, ..Default::default() });

    let attr = parse_quote!(#[repr(transparent)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(
      repr,
      Representation { repr: Repr::Transparent, ..Default::default() }
    );

    let attr = parse_quote!(#[repr(u8)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(
      repr,
      Representation {
        repr: Repr::Integer(IntegerRepr::U8),
        ..Default::default()
      }
    );

    let attr = parse_quote!(#[repr(packed)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { packed: Some(1), ..Default::default() });

    let attr = parse_quote!(#[repr(packed(1))]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { packed: Some(1), ..Default::default() });

    let attr = parse_quote!(#[repr(packed(2))]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { packed: Some(2), ..Default::default() });

    let attr = parse_quote!(#[repr(align(2))]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { align: Some(2), ..Default::default() });
  }

  #[test]
  fn parse_advanced_repr() {
    let attr = parse_quote!(#[repr(align(4), align(2))]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(repr, Representation { align: Some(4), ..Default::default() });

    let attr1 = parse_quote!(#[repr(align(1))]);
    let attr2 = parse_quote!(#[repr(align(4))]);
    let attr3 = parse_quote!(#[repr(align(2))]);
    let repr = get_repr(&[attr1, attr2, attr3]).unwrap();
    assert_eq!(repr, Representation { align: Some(4), ..Default::default() });

    let attr = parse_quote!(#[repr(C, u8)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(
      repr,
      Representation {
        repr: Repr::CWithDiscriminant(IntegerRepr::U8),
        ..Default::default()
      }
    );

    let attr = parse_quote!(#[repr(u8, C)]);
    let repr = get_repr(&[attr]).unwrap();
    assert_eq!(
      repr,
      Representation {
        repr: Repr::CWithDiscriminant(IntegerRepr::U8),
        ..Default::default()
      }
    );
  }
}

pub fn bytemuck_crate_name(input: &DeriveInput) -> TokenStream {
  const ATTR_NAME: &'static str = "crate";

  let mut crate_name = quote!(::bytemuck);
  for attr in &input.attrs {
    if !attr.path().is_ident("bytemuck") {
      continue;
    }

    attr.parse_nested_meta(|meta| {
      if meta.path.is_ident(ATTR_NAME) {
        let expr: syn::Expr = meta.value()?.parse()?;
        let mut value = &expr;
        while let syn::Expr::Group(e) = value {
          value = &e.expr;
        }
        if let syn::Expr::Lit(syn::ExprLit {
          lit: syn::Lit::Str(lit), ..
        }) = value
        {
          let suffix = lit.suffix();
          if !suffix.is_empty() {
            bail!(format!("Unexpected suffix `{}` on string literal", suffix))
          }
          let path: syn::Path = match lit.parse() {
            Ok(path) => path,
            Err(_) => {
              bail!(format!("Failed to parse path: {:?}", lit.value()))
            }
          };
          crate_name = path.into_token_stream();
        } else {
          bail!(
            "Expected bytemuck `crate` attribute to be a string: `crate = \"...\"`",
          )
        }
      }
      Ok(())
    }).unwrap();
  }

  return crate_name;
}

const GENERATED_TYPE_DOCUMENTATION: &str =
  " `bytemuck`-generated type for internal purposes only.";
