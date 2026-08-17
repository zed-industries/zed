use proc_macro2::{Literal, Punct, Spacing, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Error, Lit, LitInt, Meta, NestedMeta};

#[derive(Clone, Copy)]
pub enum IntRepr {
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl ToTokens for IntRepr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::I8 => tokens.append_all(quote! { i8 }),
            Self::I16 => tokens.append_all(quote! { i16 }),
            Self::I32 => tokens.append_all(quote! { i32 }),
            Self::I64 => tokens.append_all(quote! { i64 }),
            Self::I128 => tokens.append_all(quote! { i128 }),
            Self::U8 => tokens.append_all(quote! { u8 }),
            Self::U16 => tokens.append_all(quote! { u16 }),
            Self::U32 => tokens.append_all(quote! { u32 }),
            Self::U64 => tokens.append_all(quote! { u64 }),
            Self::U128 => tokens.append_all(quote! { u128 }),
        }
    }
}

impl IntRepr {
    #[inline]
    #[cfg(not(feature = "arbitrary_enum_discriminant"))]
    pub fn enum_discriminant(&self, _: usize) -> Option<EnumDiscriminant> {
        None
    }

    #[inline]
    #[cfg(feature = "arbitrary_enum_discriminant")]
    pub fn enum_discriminant(&self, index: usize) -> EnumDiscriminant {
        #[cfg(not(any(
            all(target_endian = "little", feature = "archive_be"),
            all(target_endian = "big", feature = "archive_le"),
        )))]
        let value = index as u128;

        #[cfg(any(
            all(target_endian = "little", feature = "archive_be"),
            all(target_endian = "big", feature = "archive_le"),
        ))]
        let value = match self {
            Self::I8 => (index as i8).swap_bytes() as u128,
            Self::I16 => (index as i16).swap_bytes() as u128,
            Self::I32 => (index as i32).swap_bytes() as u128,
            Self::I64 => (index as i64).swap_bytes() as u128,
            Self::I128 => (index as i128).swap_bytes() as u128,
            Self::U8 => (index as u8).swap_bytes() as u128,
            Self::U16 => (index as u16).swap_bytes() as u128,
            Self::U32 => (index as u32).swap_bytes() as u128,
            Self::U64 => (index as u64).swap_bytes() as u128,
            Self::U128 => (index as u128).swap_bytes(),
        };

        EnumDiscriminant { repr: *self, value }
    }
}

// None of these variants are constructed unless the arbitrary_enum_discriminant feature is enabled
#[allow(dead_code)]
pub struct EnumDiscriminant {
    repr: IntRepr,
    value: u128,
}

impl ToTokens for EnumDiscriminant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new('=', Spacing::Alone));
        tokens.append(match self.repr {
            IntRepr::I8 => Literal::i8_suffixed(self.value as i8),
            IntRepr::I16 => Literal::i16_suffixed(self.value as i16),
            IntRepr::I32 => Literal::i32_suffixed(self.value as i32),
            IntRepr::I64 => Literal::i64_suffixed(self.value as i64),
            IntRepr::I128 => Literal::i128_suffixed(self.value as i128),
            IntRepr::U8 => Literal::u8_suffixed(self.value as u8),
            IntRepr::U16 => Literal::u16_suffixed(self.value as u16),
            IntRepr::U32 => Literal::u32_suffixed(self.value as u32),
            IntRepr::U64 => Literal::u64_suffixed(self.value as u64),
            IntRepr::U128 => Literal::u128_suffixed(self.value),
        });
    }
}

#[derive(Clone, Copy)]
pub enum BaseRepr {
    C,
    // structs only
    Transparent,
    // enums only
    Int(IntRepr),
}

impl ToTokens for BaseRepr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            BaseRepr::C => tokens.append_all(quote! { C }),
            BaseRepr::Transparent => tokens.append_all(quote! { transparent }),
            BaseRepr::Int(int_repr) => tokens.append_all(quote! { #int_repr }),
        }
    }
}

#[derive(Clone)]
pub enum Modifier {
    // structs only
    Packed,
    Align(LitInt),
}

impl ToTokens for Modifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Modifier::Packed => tokens.append_all(quote! { packed }),
            Modifier::Align(n) => tokens.append_all(quote! { align(#n) }),
        }
    }
}

#[derive(Clone, Default)]
pub struct Repr {
    pub base_repr: Option<(BaseRepr, Span)>,
    pub modifier: Option<(Modifier, Span)>,
}

impl Repr {
    fn try_set_modifier<S: ToTokens>(
        &mut self,
        modifier: Modifier,
        spanned: S,
    ) -> Result<(), Error> {
        if self.modifier.is_some() {
            Err(Error::new_spanned(
                spanned,
                "only one repr modifier may be specified",
            ))
        } else {
            self.modifier = Some((modifier, spanned.span()));
            Ok(())
        }
    }

    fn try_set_base_repr<S: ToTokens>(&mut self, repr: BaseRepr, spanned: S) -> Result<(), Error> {
        if self.base_repr.is_some() {
            Err(Error::new_spanned(
                spanned,
                "only one repr may be specified",
            ))
        } else {
            self.base_repr = Some((repr, spanned.span()));
            Ok(())
        }
    }

    pub fn parse_args<'a>(
        &mut self,
        args: impl Iterator<Item = &'a NestedMeta>,
    ) -> Result<(), Error> {
        for arg in args {
            if let NestedMeta::Meta(meta) = arg {
                match meta {
                    Meta::Path(path) => {
                        if path.is_ident("packed") {
                            self.try_set_modifier(Modifier::Packed, path)?;
                        } else {
                            let parsed_repr = if path.is_ident("transparent") {
                                BaseRepr::Transparent
                            } else if path.is_ident("C") {
                                BaseRepr::C
                            } else if path.is_ident("i8") {
                                BaseRepr::Int(IntRepr::I8)
                            } else if path.is_ident("i16") {
                                BaseRepr::Int(IntRepr::I16)
                            } else if path.is_ident("i32") {
                                BaseRepr::Int(IntRepr::I32)
                            } else if path.is_ident("i64") {
                                BaseRepr::Int(IntRepr::I64)
                            } else if path.is_ident("i128") {
                                BaseRepr::Int(IntRepr::I128)
                            } else if path.is_ident("u8") {
                                BaseRepr::Int(IntRepr::U8)
                            } else if path.is_ident("u16") {
                                BaseRepr::Int(IntRepr::U16)
                            } else if path.is_ident("u32") {
                                BaseRepr::Int(IntRepr::U32)
                            } else if path.is_ident("u64") {
                                BaseRepr::Int(IntRepr::U64)
                            } else if path.is_ident("u128") {
                                BaseRepr::Int(IntRepr::U128)
                            } else {
                                return Err(Error::new_spanned(
                                    path,
                                    "invalid repr, available reprs are transparent, C, i* and u*",
                                ));
                            };

                            self.try_set_base_repr(parsed_repr, path)?;
                        }
                    }
                    Meta::List(list) => {
                        if list.path.is_ident("align") {
                            if list.nested.len() != 1 {
                                return Err(Error::new_spanned(list, "missing arguments to align"));
                            } else if let Some(NestedMeta::Lit(Lit::Int(alignment))) =
                                list.nested.first()
                            {
                                self.try_set_modifier(
                                    Modifier::Align(alignment.clone()),
                                    alignment,
                                )?;
                            }
                        }
                    }
                    _ => return Err(Error::new_spanned(meta, "invalid repr argument")),
                }
            } else {
                return Err(Error::new_spanned(arg, "invalid repr argument"));
            }
        }

        Ok(())
    }
}

impl ToTokens for Repr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let base_repr = self.base_repr.as_ref().map(|(b, _)| b);
        let base_repr_iter = base_repr.iter();
        let modifier = self.modifier.as_ref().map(|(m, _)| m);
        let modifier_iter = modifier.iter();
        tokens.append_all(quote! { #[repr(#(#base_repr_iter,)* #(#modifier_iter,)*)] });
    }
}
