use crate::names;

use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub(super) fn define_flags(
    name: &witx::Id,
    repr: witx::IntRepr,
    record: &witx::RecordDatatype,
) -> TokenStream {
    let ident = names::type_(&name);
    let abi_repr = names::wasm_type(repr.into());
    let repr = super::int_repr_tokens(repr);

    let mut names_ = vec![];
    let mut values_ = vec![];
    for (i, member) in record.members.iter().enumerate() {
        let name = names::flag_member(&member.name);
        let value_token = Literal::usize_unsuffixed(1 << i);
        names_.push(name);
        values_.push(value_token);
    }

    quote! {
        wiggle::bitflags::bitflags! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
            pub struct #ident: #repr {
                #(const #names_ = #values_;)*
            }
        }

        impl ::std::fmt::Display for #ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(stringify!(#ident))?;
                f.write_str("(")?;
                ::std::fmt::Debug::fmt(self, f)?;
                f.write_str(" (0x")?;
                ::std::fmt::LowerHex::fmt(&self.bits(), f)?;
                f.write_str("))")?;
                Ok(())
            }
        }

        impl TryFrom<#repr> for #ident {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: #repr) -> Result<Self, wiggle::GuestError> {
                #ident::from_bits(value)
                    .ok_or(wiggle::GuestError::InvalidFlagValue(stringify!(#ident)))
            }
        }

        impl TryFrom<#abi_repr> for #ident {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: #abi_repr) -> Result<Self, wiggle::GuestError> {
                #ident::try_from(#repr::try_from(value)?)
            }
        }

        impl From<#ident> for #repr {
            #[inline]
            fn from(e: #ident) -> #repr {
                e.bits()
            }
        }

        impl wiggle::GuestType for #ident {
            #[inline]
            fn guest_size() -> u32 {
                #repr::guest_size()
            }

            #[inline]
            fn guest_align() -> usize {
                #repr::guest_align()
            }

            fn read(mem: &wiggle::GuestMemory, location: wiggle::GuestPtr<#ident>) -> Result<#ident, wiggle::GuestError> {
                use std::convert::TryFrom;
                let reprval = #repr::read(mem, location.cast())?;
                let value = #ident::try_from(reprval)?;
                Ok(value)
            }

            fn write(mem: &mut wiggle::GuestMemory, location: wiggle::GuestPtr<#ident>, val: Self) -> Result<(), wiggle::GuestError> {
                let val: #repr = #repr::from(val);
                #repr::write(mem, location.cast(), val)
            }
        }
    }
}
