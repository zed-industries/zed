use crate::names;

use proc_macro2::{Literal, TokenStream};
use quote::quote;
use witx::Layout;

pub(super) fn define_variant(
    name: &witx::Id,
    v: &witx::Variant,
    derive_std_error: bool,
) -> TokenStream {
    let ident = names::type_(name);
    let size = v.mem_size_align().size as u32;
    let align = v.mem_size_align().align;
    let contents_offset = v.payload_offset() as u32;

    let lifetime = quote!('a);
    let tag_ty = super::int_repr_tokens(v.tag_repr);

    let variants = v.cases.iter().map(|c| {
        let var_name = names::enum_variant(&c.name);
        if let Some(tref) = &c.tref {
            let var_type = names::type_ref(&tref, lifetime.clone());
            quote!(#var_name(#var_type))
        } else {
            quote!(#var_name)
        }
    });

    let read_variant = v.cases.iter().enumerate().map(|(i, c)| {
        let i = Literal::usize_unsuffixed(i);
        let variantname = names::enum_variant(&c.name);
        if let Some(tref) = &c.tref {
            let varianttype = names::type_ref(tref, lifetime.clone());
            quote! {
                #i => {
                    let variant_ptr = location.cast::<u8>().add(#contents_offset)?;
                    let variant_val = <#varianttype as wiggle::GuestType>::read(mem, variant_ptr.cast())?;
                    Ok(#ident::#variantname(variant_val))
                }
            }
        } else {
            quote! { #i => Ok(#ident::#variantname), }
        }
    });

    let write_variant = v.cases.iter().enumerate().map(|(i, c)| {
        let variantname = names::enum_variant(&c.name);
        let write_tag = quote! {
            mem.write(location.cast(), #i as #tag_ty)?;
        };
        if let Some(tref) = &c.tref {
            let varianttype = names::type_ref(tref, lifetime.clone());
            quote! {
                #ident::#variantname(contents) => {
                    #write_tag
                    let variant_ptr = location.cast::<u8>().add(#contents_offset)?;
                    <#varianttype as wiggle::GuestType>::write(mem, variant_ptr.cast(), contents)?;
                }
            }
        } else {
            quote! {
                #ident::#variantname => {
                    #write_tag
                }
            }
        }
    });

    let mut extra_derive = quote!();
    let enum_try_from = if v.cases.iter().all(|c| c.tref.is_none()) {
        let tryfrom_repr_cases = v.cases.iter().enumerate().map(|(i, c)| {
            let variant_name = names::enum_variant(&c.name);
            let n = Literal::usize_unsuffixed(i);
            quote!(#n => Ok(#ident::#variant_name))
        });
        let abi_ty = names::wasm_type(v.tag_repr.into());
        extra_derive = quote!(, Copy);
        quote! {
            impl TryFrom<#tag_ty> for #ident {
                type Error = wiggle::GuestError;
                #[inline]
                fn try_from(value: #tag_ty) -> Result<#ident, wiggle::GuestError> {
                    match value {
                        #(#tryfrom_repr_cases),*,
                        _ => Err(wiggle::GuestError::InvalidEnumValue(stringify!(#ident))),
                    }
                }
            }

            impl TryFrom<#abi_ty> for #ident {
                type Error = wiggle::GuestError;
                #[inline]
                fn try_from(value: #abi_ty) -> Result<#ident, wiggle::GuestError> {
                    #ident::try_from(#tag_ty::try_from(value)?)
                }
            }
        }
    } else {
        quote!()
    };

    let enum_from = if v.cases.iter().all(|c| c.tref.is_none()) {
        let from_repr_cases = v.cases.iter().enumerate().map(|(i, c)| {
            let variant_name = names::enum_variant(&c.name);
            let n = Literal::usize_unsuffixed(i);
            quote!(#ident::#variant_name => #n)
        });
        quote! {
            impl From<#ident> for #tag_ty {
                #[inline]
                fn from(v: #ident) -> #tag_ty {
                    match v {
                        #(#from_repr_cases),*,
                    }
                }
            }
        }
    } else {
        quote!()
    };

    let extra_derive = quote!(, PartialEq #extra_derive);

    let error_impls = if derive_std_error {
        quote! {
            impl std::fmt::Display for #ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for #ident {}
        }
    } else {
        quote!()
    };

    quote! {
        #[derive(Clone, Debug #extra_derive)]
        pub enum #ident {
            #(#variants),*
        }
        #error_impls

        #enum_try_from
        #enum_from

        impl wiggle::GuestType for #ident {
            #[inline]
            fn guest_size() -> u32 {
                #size
            }

            #[inline]
            fn guest_align() -> usize {
                #align
            }

            fn read(mem: &wiggle::GuestMemory, location: wiggle::GuestPtr<Self>)
                -> Result<Self, wiggle::GuestError>
            {
                let tag = mem.read(location.cast::<#tag_ty>())?;
                match tag {
                    #(#read_variant)*
                    _ => Err(wiggle::GuestError::InvalidEnumValue(stringify!(#ident))),
                }

            }

            fn write(mem:  &mut wiggle::GuestMemory, location: wiggle::GuestPtr<Self>, val: Self)
                -> Result<(), wiggle::GuestError>
            {
                match val {
                    #(#write_variant)*
                }
                Ok(())
            }
        }
    }
}

impl super::WiggleType for witx::Variant {
    fn impls_display(&self) -> bool {
        false
    }
}
